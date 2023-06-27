use rand::Rng;
use serde_json::json;
use std::fs;
use std::io;

fn main() {
    loop {
        println!("\tA. Generate new Key, Offset, and Salt.\n\tB. Encrypt with stored Key, Offset, and Salt.\n\tC. Decrypt with external Key, Offset, and Salt.");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read input");

        match input.trim() {
            "A" => gen(),
            "B" => encrypt(),
            "C" => decrypt(),
            _ => println!("Invalid option."),
        }
    }
}

fn encrypt() {
    let file_contents = match fs::read_to_string("pair.kos") {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("Failed to load data from file: {}", error);
            return;
        }
    };

    let parsed_data: serde_json::Value = match serde_json::from_str(&file_contents) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to parse JSON data: {}", error);
            return;
        }
    };

    let key: Vec<u8> = match parsed_data.get("k") {
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

    let offset: Vec<u8> = match parsed_data.get("o") {
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

    let salt: Vec<u8> = match parsed_data.get("s") {
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
    println!("Enter data to encrypt");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read input");

    let message = input.trim();

    let mut encrypted_message: Vec<u8> = Vec::new();

    for (i, c) in message.chars().enumerate() {
        let encrypted_char =
            c as u8 + key[i % key.len()] + offset[i % offset.len()] - salt[i % salt.len()];
        encrypted_message.push(encrypted_char);
    }

    match fs::write("message.kos", encrypted_message) {
        Ok(_) => println!("Message encrypted and saved successfully."),
        Err(error) => eprintln!("Failed to save encrypted message: {}", error),
    }
}

fn decrypt() {
    let file_contents = match fs::read_to_string("pair.kos") {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("Failed to load data from file: {}", error);
            return;
        }
    };

    let parsed_data: serde_json::Value = match serde_json::from_str(&file_contents) {
        Ok(data) => data,
        Err(error) => {
            eprintln!("Failed to parse JSON data: {}", error);
            return;
        }
    };

    let key: Vec<u8> = match parsed_data.get("k") {
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

    let offset: Vec<u8> = match parsed_data.get("o") {
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

    let salt: Vec<u8> = match parsed_data.get("s") {
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

    let encoded_message = match fs::read("message.kos") {
        Ok(contents) => contents,
        Err(error) => {
            eprintln!("Failed to load encoded message from file: {}", error);
            return;
        }
    };

    let mut decrypted_message = String::new();

    for (i, encoded_char) in encoded_message.iter().enumerate() {
        let decrypted_char = encoded_char
            .wrapping_sub(key[i % key.len()])
            .wrapping_sub(offset[i % offset.len()])
            .wrapping_add(salt[i % salt.len()]);

        decrypted_message.push(decrypted_char as char);
    }

    println!("Decrypted message: {}", decrypted_message);
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

    let json_data = serde_json::to_string(&pair).unwrap();

    match fs::write("pair.kos", json_data) {
        Ok(_) => println!("Key, Offset, and Salt generated and saved successfully."),
        Err(error) => eprintln!("Failed to save data to file: {}", error),
    }
}
