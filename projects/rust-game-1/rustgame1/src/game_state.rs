use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct GameState {
    pub player_name: String,
    pub completed_modules: Vec<String>, // e.g., ["module1", "module2"]
    pub knowledge_level: u32,          // Increases with correct answers
}

impl GameState {
    pub fn new(player_name: String) -> Self {
        GameState {
            player_name,
            completed_modules: Vec::new(),
            knowledge_level: 0,
        }
    }
}
