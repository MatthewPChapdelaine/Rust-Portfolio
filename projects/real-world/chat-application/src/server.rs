use dashmap::DashMap;
use tokio::sync::mpsc::UnboundedSender;
use tokio_tungstenite::tungstenite::protocol::Message;

use crate::db::Database;
use crate::models::{ServerMessage, ChatMessage};

pub struct ChatServer {
    clients: DashMap<String, UnboundedSender<Message>>,
    usernames: DashMap<String, String>,
    rooms: DashMap<String, Vec<String>>,
    db: Database,
}

impl ChatServer {
    pub fn new(db: Database) -> Self {
        let server = Self {
            clients: DashMap::new(),
            usernames: DashMap::new(),
            rooms: DashMap::new(),
            db,
        };

        server.create_default_rooms();
        server
    }

    fn create_default_rooms(&self) {
        self.rooms.insert("general".to_string(), Vec::new());
        self.rooms.insert("random".to_string(), Vec::new());
        self.rooms.insert("tech".to_string(), Vec::new());
        
        let db = self.db.clone();
        tokio::spawn(async move {
            let _ = db.create_room("general").await;
            let _ = db.create_room("random").await;
            let _ = db.create_room("tech").await;
        });
    }

    pub async fn add_client(&self, client_id: String, sender: UnboundedSender<Message>) {
        self.clients.insert(client_id, sender);
    }

    pub async fn remove_client(&self, client_id: &str) {
        self.clients.remove(client_id);
        self.usernames.remove(client_id);
        
        for mut room in self.rooms.iter_mut() {
            room.value_mut().retain(|id| id != client_id);
        }
    }

    pub async fn set_username(&self, client_id: &str, username: &str) {
        self.usernames.insert(client_id.to_string(), username.to_string());
    }

    pub async fn join_room(&self, client_id: &str, room: &str) {
        self.rooms
            .entry(room.to_string())
            .or_insert_with(Vec::new)
            .push(client_id.to_string());

        let db = self.db.clone();
        let room_clone = room.to_string();
        tokio::spawn(async move {
            let _ = db.create_room(&room_clone).await;
        });

        let messages = self.db.get_room_messages(room, 50).await.unwrap_or_default();
        
        for msg in messages {
            self.send_to_client(
                client_id,
                ServerMessage::Message {
                    username: msg.username,
                    content: msg.content,
                    timestamp: msg.timestamp,
                    room: msg.room,
                },
            ).await;
        }
    }

    pub async fn leave_room(&self, client_id: &str, room: &str) {
        if let Some(mut clients) = self.rooms.get_mut(room) {
            clients.retain(|id| id != client_id);
        }
    }

    pub async fn broadcast_to_room(&self, room: &str, content: &str, username: &str) {
        let msg = ServerMessage::Message {
            username: username.to_string(),
            content: content.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            room: room.to_string(),
        };

        if let Some(clients) = self.rooms.get(room) {
            for client_id in clients.iter() {
                self.send_to_client(client_id, msg.clone()).await;
            }
        }
    }

    pub async fn save_and_broadcast(&self, _client_id: &str, username: &str, content: &str, room: &str) {
        let message = ChatMessage {
            id: uuid::Uuid::new_v4().to_string(),
            room: room.to_string(),
            username: username.to_string(),
            content: content.to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            is_private: false,
        };

        if let Err(e) = self.db.save_message(&message).await {
            log::error!("Failed to save message: {}", e);
        }

        self.broadcast_to_room(room, content, username).await;
    }

    pub async fn send_private_message(&self, from_id: &str, from_username: &str, to_username: &str, content: &str) {
        let to_id = self.usernames
            .iter()
            .find(|entry| entry.value() == to_username)
            .map(|entry| entry.key().clone());

        if let Some(to_id) = to_id {
            let msg = ServerMessage::PrivateMessage {
                from: from_username.to_string(),
                content: content.to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            };

            self.send_to_client(&to_id, msg.clone()).await;
            self.send_to_client(from_id, msg).await;

            let message = ChatMessage {
                id: uuid::Uuid::new_v4().to_string(),
                room: format!("private_{}_{}", from_id, to_id),
                username: from_username.to_string(),
                content: content.to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
                is_private: true,
            };

            if let Err(e) = self.db.save_message(&message).await {
                log::error!("Failed to save private message: {}", e);
            }
        } else {
            self.send_system_message(from_id, "User not found").await;
        }
    }

    pub async fn send_system_message(&self, client_id: &str, content: &str) {
        let msg = ServerMessage::SystemMessage {
            content: content.to_string(),
        };
        self.send_to_client(client_id, msg).await;
    }

    async fn send_to_client(&self, client_id: &str, msg: ServerMessage) {
        if let Some(sender) = self.clients.get(client_id) {
            if let Ok(json) = serde_json::to_string(&msg) {
                let _ = sender.send(Message::Text(json));
            }
        }
    }

    pub async fn list_rooms(&self) -> Vec<String> {
        self.rooms.iter().map(|entry| entry.key().clone()).collect()
    }
}
