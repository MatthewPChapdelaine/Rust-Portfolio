use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
#[serde(tag = "type")]
pub enum ClientMessage {
    SetUsername { username: String },
    JoinRoom { room: String },
    SendMessage { content: String },
    PrivateMessage { to: String, content: String },
    ListRooms,
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type")]
pub enum ServerMessage {
    Message { 
        username: String, 
        content: String, 
        timestamp: String,
        room: String,
    },
    PrivateMessage {
        from: String,
        content: String,
        timestamp: String,
    },
    SystemMessage { 
        content: String 
    },
    UserJoined { 
        username: String, 
        room: String 
    },
    UserLeft { 
        username: String, 
        room: String 
    },
    RoomsList { 
        rooms: Vec<String> 
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: String,
    pub room: String,
    pub username: String,
    pub content: String,
    pub timestamp: String,
    pub is_private: bool,
}

#[derive(Debug, Clone)]
pub struct Client {
    pub id: String,
    pub username: Option<String>,
    pub current_room: Option<String>,
}
