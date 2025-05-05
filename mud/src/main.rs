pub mod engine;
use engine::entities;
use engine::server::launch_server;

use std::collections::HashMap;
use configparser::ini::Ini;

fn main() {
  let mut config: Ini = Ini::new();
  println!("Welcome to the Rust MUD!");
  println!("Type 'help' for a list of commands.");
  let _map: HashMap<String, HashMap<String, Option<String>>> = config.load("docs/config.ini").expect("Failed to load config file");
  println!("Config Map: \n{:?}", _map); 
  println!("{:?}", config.get("Admin", "logo_file_path").unwrap());
  // let _ = launch_server();
  let critter = entities::create_mob();
  println!("{:?}", critter);
}