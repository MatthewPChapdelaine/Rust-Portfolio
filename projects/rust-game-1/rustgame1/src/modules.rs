use crate::game_state::GameState;
use crate::input_handler::get_input;

pub mod module1 {
    use super::*;

    pub fn run(game_state: &mut GameState) {
        println!("=== Module 1: Property and Labor Structure ===");
        // Introduction
        println!("Learn to design housing and labor systems for workers.");
        // Lecture
        println!("Company-owned housing can reduce worker costs but may limit freedom...");
        // Interaction
        let decision = get_input("Provide free housing? (yes/no)");
        if decision == "yes" {
            println!("Workers are happy, but company costs rise.");
        } else {
            println!("Workers pay rent, increasing their stress.");
        }
        // Assessment
        let answer = get_input("Name one benefit of company housing:");
        if answer.contains("cost") || answer.contains("free") {
            println!("Correct! Knowledge level increased.");
            game_state.knowledge_level += 1;
        } else {
            println!("Incorrect. A benefit is reduced costs for workers.");
        }
        game_state.completed_modules.push("module1".to_string());
    }
}

pub mod module2 {
    use super::*;
    pub fn run(game_state: &mut GameState) {
        println!("=== Module 2: University Integration ===");
        println!("Explore how universities and companies collaborate...");
        // Similar structure: intro, lecture, interaction, assessment
        game_state.completed_modules.push("module2".to_string());
    }
}

pub mod module3 {
    use super::*;
    pub fn run(game_state: &mut GameState) {
        println!("=== Module 3: Sustainable City Design ===");
        println!("Design a radial city with sustainable systems...");
        let choice = get_input("Choose energy: solar, wind, or coal?");
        match choice.as_str() {
            "solar" => println!("Good choice! Low emissions."),
            "wind" => println!("Nice! Renewable but noisy."),
            _ => println!("Coal harms sustainability."),
        }
        game_state.completed_modules.push("module3".to_string());
    }
}

pub mod module4 {
    use super::*;
    pub fn run(game_state: &mut GameState) {
        println!("=== Module 4: Metaverse: The Nexus ===");
        println!("Enter a virtual world inspired by 'Tenchi Muyo'...");
        let action = get_input("Visit Galaxy Police or Jurai Temple?");
        if action.contains("police") {
            println!("You learn about space law enforcement.");
        } else {
            println!("You uncover Jurai’s ancient secrets.");
        }
        game_state.completed_modules.push("module4".to_string());
    }
}

pub mod module5 {
    use super::*;
    pub fn run(game_state: &mut GameState) {
        println!("=== Module 5: Interconnected Systems Theory (IST) ===");
        println!("Study how systems interact mathematically and philosophically...");
        let tweak = get_input("Increase city energy output? (yes/no)");
        if tweak == "yes" {
            println!("Output rises, but waste increases too.");
        }
        game_state.completed_modules.push("module5".to_string());
    }
}
