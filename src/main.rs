use serde::Deserialize;
use std::{env, fs};

#[derive(Deserialize)]
struct User {
    name: String,
    age: u8,
}

fn main() {
    // 1. Reading arguments
    let args: Vec<String> = env::args().collect();
    let file_path = args.get(1).expect("Usage: cargo run -- <file.json>");

    // 2. Reading file
    let contents = fs::read_to_string(file_path).expect("Failed to read file!!!!");

    // 3. Parsing JSON
    let users: Vec<User> = serde_json::from_str(&contents).expect("Invalid JSON!!!!!!");

    // 4. Iterator pipeline
    users
        .iter()
        .filter(|u| u.age >= 18)
        .map(|u| format!("User: {}, Age: {}", u.name, u.age))
        .for_each(|line| println!("{}", line));
}
