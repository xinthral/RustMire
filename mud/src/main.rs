use std::fs;
use configparser::ini::Ini;
use std::collections::HashMap;

pub fn read_in_ascii_logo(filename: &str) -> Result<String, Box<dyn std::error::Error>> {
  let contents = fs::read_to_string(filename)?;
  Ok(contents)
}

pub fn print_help_menu() {
  println!("Available commands:");
  println!("- help: Display this help menu");
  // println!("- look: Examine the current room");
  // println!("- go [direction]: Move in a specified direction (north, south, east, west)");
  // println!("- take [item]: Pick up an item from the current room");
  // println!("- inventory: List all items in your inventory");
  // println!("- quit: Exit the game");
  // println!("- save: Save your game state");
  // println!("- load: Load a saved game state");
  // println!("- hint: Provide a hint for the current room");
  // println!("- map: Display a map of the game world");
  // println!("- stats: Display your character's statistics");
  // println!("- quest: Display the current quest");
  // println!("- quest_complete: Complete the current quest");
  // println!("- quest_fail: Fail the current quest");
  // println!("- quest_hint: Provide a hint for the current quest");
  // println!("- quest_reward: Receive the quest reward");
  // println!("- quest_status: Check the current quest status");
  
}

fn main() {
  let mut config: Ini = Ini::new();
  println!("Welcome to the Rust MUD!");
  println!("Type 'help' for a list of commands.");
  let _: HashMap<String, HashMap<String, Option<String>>> = config.load("docs/config.ini").expect("Failed to load config file");
  // println!("Config Map: \n{:?}", map); 
  // println!("{:?}", config.get("Admin", "logo_file_path").unwrap());
  let logo: String = read_in_ascii_logo("media/ascii_logo.txt").expect("Failed to read logo file");
  println!("{}", logo);
}