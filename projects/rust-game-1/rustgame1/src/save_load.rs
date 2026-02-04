use std::fs::File;
use std::io::{Write, Read};
use serde_json;
use crate::game_state::GameState;

pub fn save_game(game_state: &GameState) {
    let json = serde_json::to_string(game_state).unwrap();
    let mut file = File::create("save.json").unwrap();
    file.write_all(json.as_bytes()).unwrap();
    println!("Game saved successfully!");
}

pub fn load_game() -> Option<GameState> {
    match File::open("save.json") {
        Ok(mut file) => {
            let mut json = String::new();
            file.read_to_string(&mut json).unwrap();
            match serde_json::from_str(&json) {
                Ok(game_state) => Some(game_state),
                Err(_) => {
                    println!("Corrupted save file. Starting new game.");
                    None
                }
            }
        }
        Err(_) => {
            println!("No save file found. Starting new game.");
            None
        }
    }
}
