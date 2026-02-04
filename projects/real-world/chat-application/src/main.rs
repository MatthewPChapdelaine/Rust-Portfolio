use tokio::net::{TcpListener, TcpStream};
use tokio_tungstenite::tungstenite::protocol::Message;
use tokio_tungstenite::accept_async;
use futures_util::{StreamExt, SinkExt};
use std::sync::Arc;

mod server;
mod models;
mod db;

use server::ChatServer;
use models::ClientMessage;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let database_url = "sqlite://chat.db";
    let db = db::Database::new(database_url).await?;
    db.init().await?;
    
    log::info!("Database initialized");

    let server = Arc::new(ChatServer::new(db));
    let addr = "127.0.0.1:9001";
    let listener = TcpListener::bind(addr).await?;

    log::info!("WebSocket server listening on: ws://{}", addr);
    log::info!("Open client/index.html in your browser to connect");

    while let Ok((stream, peer_addr)) = listener.accept().await {
        log::info!("New connection from: {}", peer_addr);
        let server = Arc::clone(&server);
        
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream, server).await {
                log::error!("Error handling connection: {}", e);
            }
        });
    }

    Ok(())
}

async fn handle_connection(
    stream: TcpStream,
    server: Arc<ChatServer>,
) -> Result<(), Box<dyn std::error::Error>> {
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let client_id = uuid::Uuid::new_v4().to_string();
    let mut username: Option<String> = None;
    let mut current_room: Option<String> = None;

    log::info!("Client {} connected", client_id);

    server.send_system_message(
        &client_id,
        "Welcome to the chat! Please set your username with: /nick YourName"
    ).await;

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();
    server.add_client(client_id.clone(), tx).await;

    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if ws_sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    while let Some(msg) = ws_receiver.next().await {
        let msg = match msg {
            Ok(msg) => msg,
            Err(e) => {
                log::error!("WebSocket error: {}", e);
                break;
            }
        };

        if let Message::Text(text) = msg {
            if let Ok(client_msg) = serde_json::from_str::<ClientMessage>(&text) {
                match client_msg {
                    ClientMessage::SetUsername { username: name } => {
                        username = Some(name.clone());
                        server.set_username(&client_id, &name).await;
                        server.send_system_message(&client_id, &format!("Username set to: {}", name)).await;
                    }
                    ClientMessage::JoinRoom { room } => {
                        if username.is_none() {
                            server.send_system_message(&client_id, "Please set username first").await;
                            continue;
                        }

                        if let Some(old_room) = &current_room {
                            server.leave_room(&client_id, old_room).await;
                        }

                        server.join_room(&client_id, &room).await;
                        current_room = Some(room.clone());
                        
                        if let Some(ref user) = username {
                            server.broadcast_to_room(
                                &room,
                                &format!("{} joined the room", user),
                                "system"
                            ).await;
                        }
                    }
                    ClientMessage::SendMessage { content } => {
                        if let (Some(ref user), Some(ref room)) = (&username, &current_room) {
                            server.save_and_broadcast(&client_id, user, &content, room).await;
                        } else {
                            server.send_system_message(&client_id, "Join a room first").await;
                        }
                    }
                    ClientMessage::PrivateMessage { to, content } => {
                        if let Some(ref user) = username {
                            server.send_private_message(&client_id, user, &to, &content).await;
                        }
                    }
                    ClientMessage::ListRooms => {
                        let rooms = server.list_rooms().await;
                        let msg = format!("Available rooms: {}", rooms.join(", "));
                        server.send_system_message(&client_id, &msg).await;
                    }
                }
            }
        } else if let Message::Close(_) = msg {
            break;
        }
    }

    if let Some(room) = &current_room {
        server.leave_room(&client_id, room).await;
        if let Some(user) = &username {
            server.broadcast_to_room(room, &format!("{} left the room", user), "system").await;
        }
    }

    server.remove_client(&client_id).await;
    send_task.abort();
    
    log::info!("Client {} disconnected", client_id);

    Ok(())
}
