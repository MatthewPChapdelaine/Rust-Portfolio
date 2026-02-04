use rand::Rng;
use std::io;
use std::collections::HashMap;

// Represents the player's current state
#[derive(Debug)]
enum State {
    InSpace,      // Player is in space and operational
    Grounded,     // Player is grounded due to insufficient funds
    Traveling,    // Player is traveling between star systems
}

// Represents an activity with income potential
#[derive(Clone)]
struct Activity {
    name: String,
    min_income: u32,
    max_income: u32,
}

// Represents a planet with its attributes and activities
#[derive(Clone)]
struct Planet {
    orbit_level: u32,      // 1-4
    bay_level: u32,        // 1-4
    description: String,   // Flavor text for immersion
    activities: Vec<Activity>,
}

// Represents a mission with reward and difficulty
#[derive(Clone)]
struct Mission {
    description: String,
    reward: u32,
    difficulty: u32, // 1-10 scale
}

// Game constants
const INITIAL_GRANT: u32 = 10_000;           // Starting funds
const SPACE_COST: u32 = 1_500;               // Weekly cost in space
const GROUNDED_COST: u32 = 600;              // Weekly cost when grounded
const LICENSE_RENEWAL_COST: u32 = 10_000;    // Cost to renew license

// Main game structure
struct Game {
    funds: u32,
    state: State,
    week: u32,
    current_star_system: String,
    current_planet: String,
    star_systems: HashMap<String, Vec<Planet>>,
    missions: HashMap<String, Vec<Mission>>, // Missions per planet
    travel_weeks_left: u32, // Weeks remaining for inter-system travel
}

impl Game {
    // Initialize a new game
    fn new() -> Self {
        let mut star_systems = HashMap::new();
        let mut missions = HashMap::new();

        // Define 4 star systems with 4 planets each
        let systems = vec!["Alpha", "Beta", "Gamma", "Delta"];
        for system in systems {
            let mut planets = Vec::new();
            for i in 1..=4 {
                let planet_name = format!("{system}{i}");
                let planet = Planet {
                    orbit_level: i,
                    bay_level: i,
                    description: format!("A planet in the {system} star system with orbit level {i} and bay level {i}."),
                    activities: vec![
                        Activity {
                            name: "Trading".to_string(),
                            min_income: 2000,
                            max_income: 2000,
                        },
                        Activity {
                            name: "Exploring".to_string(),
                            min_income: 1500,
                            max_income: 1500,
                        },
                    ],
                };
                planets.push(planet);

                // Add a sample mission for each planet
                missions.insert(
                    planet_name.clone(),
                    vec![Mission {
                        description: format!("Mission on {planet_name}"),
                        reward: 200 + (i * 200), // Reward scales with orbit level
                        difficulty: i,           // Difficulty scales with orbit level
                    }],
                );
            }
            star_systems.insert(system.to_string(), planets);
        }

        Game {
            funds: INITIAL_GRANT,
            state: State::InSpace,
            week: 1,
            current_star_system: "Alpha".to_string(),
            current_planet: "Alpha1".to_string(),
            star_systems,
            missions,
            travel_weeks_left: 0,
        }
    }

    // Pay weekly costs based on state
    fn pay_costs(&mut self) -> bool {
        match self.state {
            State::InSpace | State::Traveling => {
                if self.funds < SPACE_COST {
                    println!("Cannot pay space costs of {} credits. Your starship is locked in the bay by government decree.", SPACE_COST);
                    self.state = State::Grounded;
                    true
                } else {
                    self.funds -= SPACE_COST;
                    println!("Paid space costs of {} credits.", SPACE_COST);
                    true
                }
            }
            State::Grounded => {
                if self.funds < GROUNDED_COST {
                    println!("Cannot pay grounded costs of {} credits. Game over.", GROUNDED_COST);
                    false
                } else {
                    self.funds -= GROUNDED_COST;
                    println!("Paid grounded costs of {} credits.", GROUNDED_COST);
                    true
                }
            }
        }
    }

    // Choose an activity to earn income
    fn choose_activity(&mut self) {
        let planet = self.star_systems
            .get(&self.current_star_system)
            .unwrap()
            .iter()
            .find(|p| p.description.contains(&self.current_planet))
            .unwrap();
        let income = choose_activity(&planet.activities);
        self.funds += income;
    }

    // Accept and attempt a mission
    fn accept_mission(&mut self) {
        if let Some(missions) = self.missions.get(&self.current_planet) {
            if missions.is_empty() {
                println!("No missions available on {}.", self.current_planet);
                return;
            }
            println!("Available missions on {}:", self.current_planet);
            for (i, mission) in missions.iter().enumerate() {
                println!(
                    "{}. {} - Reward: {} credits, Difficulty: {}",
                    i + 1, mission.description, mission.reward, mission.difficulty
                );
            }
            println!("Choose a mission (1-{}) or 0 to skip:", missions.len());
            let choice = read_input_as_number();
            if choice == 0 || choice > missions.len() {
                println!("Skipping missions.");
                return;
            }
            let mission = &missions[choice - 1];
            let success_chance = 100 - (mission.difficulty * 10);
            let roll = rand::thread_rng().gen_range(0..100);
            if roll < success_chance {
                println!("Mission successful! Earned {} credits.", mission.reward);
                self.funds += mission.reward;
            } else {
                println!("Mission failed. No reward.");
            }
        }
    }

    // Handle travel between planets and star systems
    fn travel(&mut self) {
        if matches!(self.state, State::Grounded) {
            println!("You are grounded and cannot travel.");
            return;
        }
        println!("Available star systems: {:?}", self.star_systems.keys().collect::<Vec<_>>());
        println!("Enter the name of the star system to travel to:");
        let system_input = read_input_as_string();
        if !self.star_systems.contains_key(&system_input) {
            println!("Invalid star system.");
            return;
        }
        println!(
            "Available planets in {}: {:?}",
            system_input,
            self.star_systems
                .get(&system_input)
                .unwrap()
                .iter()
                .map(|p| p.description.split_whitespace().last().unwrap())
                .collect::<Vec<_>>()
        );
        println!("Enter the name of the planet to travel to:");
        let planet_input = read_input_as_string();
        let planet = self.star_systems
            .get(&system_input)
            .unwrap()
            .iter()
            .find(|p| p.description.contains(&planet_input));
        if planet.is_none() {
            println!("Invalid planet.");
            return;
        }
        if system_input == self.current_star_system {
            self.current_planet = planet_input.clone();
            println!(
                "Traveled to {} in the {} star system.",
                self.current_planet, self.current_star_system
            );
        } else {
            self.state = State::Traveling;
            self.travel_weeks_left = 1;
            self.current_star_system = system_input.clone();
            self.current_planet = planet_input.clone();
            println!(
                "Traveling to {} in the {} star system. Arrival in 1 week.",
                self.current_planet, self.current_star_system
            );
        }
    }

    // Check and renew license if grounded
    fn check_license_renewal(&mut self) {
        if matches!(self.state, State::Grounded) && self.funds >= LICENSE_RENEWAL_COST {
            println!("You have enough funds to renew your license. Do you want to? (y/n)");
            if read_yes_no() {
                self.funds -= LICENSE_RENEWAL_COST;
                self.state = State::InSpace;
                println!("License renewed. You are back in space.");
            }
        }
    }

    // Advance to the next week
    fn advance_week(&mut self) {
        self.week += 1;
        if matches!(self.state, State::Traveling) {
            self.travel_weeks_left -= 1;
            if self.travel_weeks_left == 0 {
                self.state = State::InSpace;
                println!(
                    "Arrived at {} in the {} star system.",
                    self.current_planet, self.current_star_system
                );
            }
        }
    }
}

// Utility function to read string input
fn read_input_as_string() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    input.trim().to_string()
}

// Utility function to read numeric input
fn read_input_as_number() -> usize {
    loop {
        let input = read_input_as_string();
        match input.parse::<usize>() {
            Ok(num) => return num,
            Err(_) => println!("Please enter a valid number."),
        }
    }
}

// Utility function to read yes/no input
fn read_yes_no() -> bool {
    loop {
        let input = read_input_as_string().to_lowercase();
        match input.as_str() {
            "y" | "yes" => return true,
            "n" | "no" => return false,
            _ => println!("Please enter y or n."),
        }
    }
}

// Choose an activity and return earned income
fn choose_activity(activities: &[Activity]) -> u32 {
    println!("Choose an activity:");
    for (i, activity) in activities.iter().enumerate() {
        println!(
            "{}. {} - Income: {}-{} credits",
            i + 1, activity.name, activity.min_income, activity.max_income
        );
    }
    loop {
        let choice = read_input_as_number();
        if choice >= 1 && choice <= activities.len() {
            let activity = &activities[choice - 1];
            let income = if activity.min_income == activity.max_income {
                activity.min_income
            } else {
                rand::thread_rng().gen_range(activity.min_income..=activity.max_income)
            };
            println!("Earned {} credits from {}.", income, activity.name);
            return income;
        }
        println!("Invalid choice, please try again.");
    }
}

// Main game loop
fn main() {
    let mut game = Game::new();
    println!("Welcome to Orbspace! You start with {} credits.", INITIAL_GRANT);
    loop {
        println!(
            "\nWeek {}, State: {:?}, Star System: {}, Planet: {}, Funds: {}",
            game.week, game.state, game.current_star_system, game.current_planet, game.funds
        );
        if !game.pay_costs() {
            break;
        }
        if matches!(game.state, State::Traveling) {
            println!("Traveling... {} weeks left.", game.travel_weeks_left);
        } else {
            game.travel();
            if !matches!(game.state, State::Traveling) {
                game.choose_activity();
                game.accept_mission();
            }
        }
        game.check_license_renewal();
        println!("Continue to next week? (y/n)");
        if !read_yes_no() {
            println!(
                "Game ended. Final funds: {} credits after {} weeks.",
                game.funds, game.week
            );
            break;
        }
        game.advance_week();
    }
}
