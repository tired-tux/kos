use rand::Rng;
use serde_json::*;
use std::fs;
use std::io;

fn main() {
    println!("\tA. Generate new Key, Offset, and Salt.\n\tB. Encrypt with stored Key, Offset, and Salt.\n\tC. Decrypt with external Key, Offset, and Salt.");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");
    if input.trim() == "B" {
        encrypt();
    }
    if input.trim() == "A" {
        gen();
    }
}

fn encrypt() {
    // Load data from a file
    let file_contents = match fs::read_to_string("pair.kos") {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("Failed to load data from file: {}", error);
            return;
        }
    };

    // Parse JSON data
    let parsed_data: serde_json::Value = match serde_json::from_str(&file_contents) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to parse JSON data: {}", error);
            return;
        }
    };

    // Extract key, offset, and salt
    let key = match parsed_data.get("k") {
        Some(value) => match value.as_array() {
            Some(arr) => arr.iter().map(|v| v.as_u64().unwrap() as u8).collect(),
            None => {
                eprintln!("Invalid key data");
                return;
            }
        },
        None => {
            eprintln!("Key not found in JSON data");
            return;
        }
    };
    let offset = match parsed_data.get("o") {
        Some(value) => match value.as_array() {
            Some(arr) => arr.iter().map(|v| v.as_u64().unwrap() as u8).collect(),
            None => {
                eprintln!("Invalid offset data");
                return;
            }
        },
        None => {
            eprintln!("Offset not found in JSON data");
            return;
        }
    };
    let salt = match parsed_data.get("s") {
        Some(value) => match value.as_array() {
            Some(arr) => arr.iter().map(|v| v.as_u64().unwrap() as u8).collect(),
            None => {
                eprintln!("Invalid salt data");
                return;
            }
        },
        None => {
            eprintln!("Salt not found in JSON data");
            return;
        }
    };

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let text = input.trim();

    let mut message: Vec<u8> = Vec::new();

    for (i, c) in text.chars().enumerate() {
        let encrypted_char = (c as u8).wrapping_add(key[i % key.len()])
            .wrapping_sub(offset[i % offset.len()])
            .wrapping_sub(salt[i % salt.len()]);
        message.push(encrypted_char);
    }

    // Save the encrypted message to a file
    if let Err(error) = fs::write("message.kos", &message) {
        eprintln!("Failed to save encrypted message to file: {}", error);
        return;
    }

    println!("Message encrypted successfully.");
}

fn gen() {
    let mut rng = rand::thread_rng();

    let key: Vec<u8> = (0..1024).map(|_| rng.gen_range(1..=50)).collect();
    let offset: Vec<u8> = (0..1024).map(|_| rng.gen_range(1..=50)).collect();
    let salt: Vec<u8> = (0..1024).map(|_| rng.gen_range(1..=50)).collect();

    let pair = json!({
        "k": key,
        "o": offset,
        "s": salt
    });

    // Convert the data to JSON format
    let json_data = serde_json::to_string(&pair).unwrap();

    // Save the JSON data to a file
    if let Err(error) = fs::write("pair.kos", json_data) {
        eprintln!("Failed to save data to file: {}", error);
        return;
    }

    println!("Data saved successfully.");
}
