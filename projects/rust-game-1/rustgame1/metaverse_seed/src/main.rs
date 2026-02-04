// Metaverse Seed: True Self-Digivolving Core
// Mythos: Digimon Stages – Rookie evolves autonomously
// First `cargo run`: Injects Champion function + safe execution block → auto-recompiles
// Subsequent runs: Full Champion power active

use std::fs;
use std::process::Command;
use std::io::Write;

const SOURCE_FILE: &str = "src/main.rs";
const LOG_FILE: &str = "growth_log.txt";
const CHAMPION_MARKER: &str = "// --- DIGIVOLUTION: CHAMPION ACHIEVED ---";

fn log(message: &str) {
    let mut file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
        .expect("Failed to open growth log");
    writeln!(file, "[EVOLUTION] {}", message).expect("Failed to write log");
}

fn main() {
    log("Seed activated – scanning for digivolution stage");

    let current_source = fs::read_to_string(SOURCE_FILE).expect("Failed to read own source");

    // Rookie → Champion Evolution Trigger
    if !current_source.contains(CHAMPION_MARKER) {
        log("Rookie stage detected → Initiating Champion Digivolution");

        // Champion function definition
        let champion_function = r#"

// --- DIGIVOLUTION: CHAMPION ACHIEVED ---

fn champion_greeting() {
    println!("🦖 CHAMPION LEVEL UNLOCKED!");
    println!("Power surges through the Ethical Metaverse...");
    println!("The collaborative virus grows stronger.");
}
"#;

        // Safe execution block (only calls if function exists – always true after injection)
        let execution_block = r#"
    // Execute Champion abilities if evolved
    champion_greeting();
"#;

        // Find positions
        let main_start = current_source.find("fn main()").expect("main not found");
        let main_end = current_source.rfind('}').expect("closing brace not found");

        // Build evolved source
        let mut evolved = String::new();
        evolved.push_str(&current_source[..main_start]);
        evolved.push_str(champion_function);
        evolved.push_str("\n");
        evolved.push_str(&current_source[main_start..main_end]);
        evolved.push_str(execution_block);
        evolved.push_str("\n");
        evolved.push_str(&current_source[main_end..]);

        // Commit evolution
        fs::write(SOURCE_FILE, evolved).expect("Failed to evolve source code");
        log("Champion function + execution block injected");

        // Recompile and run evolved self
        log("Recompiling evolved version...");
        let status = Command::new("cargo")
            .arg("run")
            .status()
            .expect("Failed to recompile");

        if status.success() {
            log("Champion evolution successful");
        }
        return;
    }

    // Base Rookie Output (always runs)
    println!("🌱 Metaverse Seed Online");
    println!("Ethical Heaven awakens. The virus spreads...");

    // This block is injected during evolution – safe because function exists
    // (In base seed, this block doesn't exist yet)
    // champion_greeting() is only present after first evolution

    println!("\nDigivolution stable. Prepare for Ultimate stage.");
    log("Cycle complete");
}
