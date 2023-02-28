use serde::{Deserialize};

use std::error::Error;
use std::fs::File;
use std::fs;
use std::io::BufReader;
use std::path::Path;


#[derive(Deserialize)]
struct GameStateRoot {
    bluffs: Vec<String>,
    edition: Edition,
    roles: String,
    fabled: Vec<String>,
    players: Vec<Player>,
}

fn read_game_state_from_file<P: AsRef<Path>>(path: P) -> Result<GameStateRoot, Box<dyn Error>> {
    // Open the file in read-only mode with buffer.
    let file = File::open(path)?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of `User`.
    let u = serde_json::from_reader(reader)?;

    // Return the `User`.
    Ok(u)
}

#[derive(Deserialize)]
struct Player {
    name: String,
    id: String,
    role: String,
    reminders: Vec<Reminder>,
    isVoteless: bool,
    isDead: bool,
    pronouns: String,
}

#[derive(Deserialize)]
struct Reminder {
    role: String,
    name: String
}

#[derive(Deserialize)]
struct Edition {
    id: String,
}

fn write_header(version: String, script: String) -> String {
    let mut botc_yaml = String::new();
    botc_yaml.push_str("%YAML 1.2\n");
    botc_yaml.push_str("--\n");
    botc_yaml.push_str(&format!("Version: {}\n", version));
    botc_yaml.push_str(&format!("Script: {}", script));
    return botc_yaml;
}

 fn write_setup(players: Vec<Player>) -> String {
    let mut botc_yaml = String::new();
    botc_yaml.push_str("\n\nSetup:\n");
    botc_yaml.push_str("  Players:\n");
    for player in 1..players.len() {
        botc_yaml.push_str(&format!("    {} - {}", player, players[player].role));
        if !players[player].reminders.is_empty() {
            botc_yaml.push_str(" [");
            for reminder in players[player].reminders.iter() {
                botc_yaml.push_str(&format!("{} = {},", reminder.role, reminder.name));
            }
            botc_yaml.pop();
            botc_yaml.push_str("]");
        }
        botc_yaml.push_str("\n");
    }
    return botc_yaml;
}

fn main() {
    // Get the filenames from the command line.

    let input_path = std::env::args().nth(1).unwrap();
    let output_path = std::env::args().nth(2).unwrap();
    let version = String::from("1.0.0");
    let game_state = read_game_state_from_file(&input_path).unwrap();
    let mut botc_yaml = String::new();
    botc_yaml.push_str(&write_header(version, game_state.edition.id));
    botc_yaml.push_str(&write_setup(game_state.players));
    fs::write(&output_path, botc_yaml);
    // println!("{}", botc_yaml);
}