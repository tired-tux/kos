extern crate rand;
extern crate serde_json;
use rand::Rng;
use serde_json::json;
use std::fs;
use string_join::Join;
pub fn encrypt(message: &str, file: &str) {
   let fiel = ".kos".join(&[file]);
	 let file_contents = match fs::read_to_string("pair.kos") {
        Ok(contents) => contents,
        Err(_error) => {
            eprintln!("");
            return;
        }
    };

    let parsed_data: serde_json::Value = match serde_json::from_str(&file_contents) {
        Ok(data) => data,
        Err(_error) => {
            eprintln!("");
            return;
        }
    };

    let key: Vec<u8> = match parsed_data.get("k") {
        Some(value) => match value.as_array() {
            Some(arr) => arr.iter().map(|v| v.as_u64().unwrap() as u8).collect(),
            None => {
                eprintln!("");
                return;
            }
        },
        None => {
            eprintln!("");
            return;
        }
    };

    let offset: Vec<u8> = match parsed_data.get("o") {
        Some(value) => match value.as_array() {
            Some(arr) => arr.iter().map(|v| v.as_u64().unwrap() as u8).collect(),
            None => {
                eprintln!("");
                return;
            }
        },
        None => {
            eprintln!("");
            return;
        }
    };

    let salt: Vec<u8> = match parsed_data.get("s") {
        Some(value) => match value.as_array() {
            Some(arr) => arr.iter().map(|v| v.as_u64().unwrap() as u8).collect(),
            None => {
                eprintln!("");
                return;
            }
        },
        None => {
            eprintln!("");
            return;
        }
    };

    let mut encrypted_message: Vec<u8> = Vec::new();

    for (i, c) in message.chars().enumerate() {
        let encrypted_char =
            c as u8 + key[i % key.len()] + offset[i % offset.len()] - salt[i % salt.len()];
        encrypted_message.push(encrypted_char);
    }

    match fs::write(fiel, encrypted_message) {
        Ok(_) => print!(""),
        Err(_error) => eprint!(""),
    }
}
pub fn decrypt(file: &str) -> String {
    let fiel = ".kos".join(&[file]);
	let file_contents = match fs::read_to_string("pair.kos") {
        Ok(contents) => contents,
        Err(_error) => {
            eprintln!("");
            return String::new();
        }
    };

    let parsed_data: serde_json::Value = match serde_json::from_str(&file_contents) {
        Ok(data) => data,
        Err(_error) => {
            eprintln!("");
            return String::new();
        }
    };

    let key: Vec<u8> = match parsed_data.get("k") {
        Some(value) => match value.as_array() {
            Some(arr) => arr.iter().map(|v| v.as_u64().unwrap() as u8).collect(),
            None => {
                eprintln!("");
                return String::new();
            }
        },
        None => {
            eprintln!("");
            return String::new();
        }
    };

    let offset: Vec<u8> = match parsed_data.get("o") {
        Some(value) => match value.as_array() {
            Some(arr) => arr.iter().map(|v| v.as_u64().unwrap() as u8).collect(),
            None => {
                eprintln!("");
                return String::new();
            }
        },
        None => {
            eprintln!("");
            return String::new();
        }
    };

    let salt: Vec<u8> = match parsed_data.get("s") {
        Some(value) => match value.as_array() {
            Some(arr) => arr.iter().map(|v| v.as_u64().unwrap() as u8).collect(),
            None => {
                eprintln!("");
                return String::new();
            }
        },
        None => {
            eprintln!("Salt not found in JSON data");
            return String::new();
        }
    };

    let encoded_message = match fs::read(fiel) {
        Ok(contents) => contents,
        Err(_error) => {
            eprintln!("");
            return String::new();
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

    decrypted_message
}
pub fn gen() {
    let mut rng = rand::thread_rng();

    let key: Vec<u8> = (0..10240).map(|_| rng.gen_range(1..=50)).collect();
    let offset: Vec<u8> = (0..10240).map(|_| rng.gen_range(1..=50)).collect();
    let salt: Vec<u8> = (0..10240).map(|_| rng.gen_range(1..=50)).collect();

    let pair = json!({
        "k": key,
        "o": offset,
        "s": salt
    });

    let json_data = serde_json::to_string(&pair).unwrap();

    match fs::write("pair.kos", json_data) {
        Ok(_) => print!(""),
        Err(_error) => eprint!(""),
    }
}
pub fn tux(str: &str) {
    if str == "bird" {
        let tux = "                                .:xxxxxxxx:.
                             .xxxxxxxxxxxxxxxx.
                            :xxxxxxxxxxxxxxxxxxx:.
                           .xxxxxxxxxxxxxxxxxxxxxxx:
                          :xxxxxxxxxxxxxxxxxxxxxxxxx:
                          xxxxxxxxxxxxxxxxxxxxxxxxxxX:
                          xxx:::xxxxxxxx::::xxxxxxxxx:
                         .xx:   ::xxxxx:     :xxxxxxxx
                         :xx  x.  xxxx:  xx.  xxxxxxxx
                         :xx xxx  xxxx: xxxx  :xxxxxxx
                         'xx 'xx  xxxx:. xx'  xxxxxxxx
                          xx ::::::xx:::::.   xxxxxxxx
                          xx:::::.::::.:::::::xxxxxxxx
                          :x'::::'::::':::::':xxxxxxxxx.
                          :xx.::::::::::::'   xxxxxxxxxx
                          :xx: '::::::::'     :xxxxxxxxxx.
                         .xx     '::::'        'xxxxxxxxxx.
                       .xxxx                     'xxxxxxxxx.
                     .xxxx                         'xxxxxxxxx.
                   .xxxxx:                          xxxxxxxxxx.
                  .xxxxx:'                          xxxxxxxxxxx.
                 .xxxxxx:::.           .       ..:::_xxxxxxxxxxx:.
                .xxxxxxx''      ':::''            ''::xxxxxxxxxxxx.
                xxxxxx            :                  '::xxxxxxxxxxxx
               :xxxx:'            :                    'xxxxxxxxxxxx:
              .xxxxx              :                     ::xxxxxxxxxxxx
              xxxx:'                                    ::xxxxxxxxxxxx
              xxxx               .                      ::xxxxxxxxxxxx.
          .:xxxxxx               :                      ::xxxxxxxxxxxx::
          xxxxxxxx               :                      ::xxxxxxxxxxxxx:
          xxxxxxxx               :                      ::xxxxxxxxxxxxx:
          ':xxxxxx               '                      ::xxxxxxxxxxxx:'
            .:. xx:.                                   .:xxxxxxxxxxxxx'
          ::::::.'xx:.            :                  .:: xxxxxxxxxxx':
  .:::::::::::::::.'xxxx.                            ::::'xxxxxxxx':::.
  ::::::::::::::::::.'xxxxx                          :::::.'.xx.'::::::.
  ::::::::::::::::::::.'xxxx:.                       :::::::.'':::::::::
  ':::::::::::::::::::::.'xx:'                     .'::::::::::::::::::::..
    :::::::::::::::::::::.'xx                    .:: :::::::::::::::::::::::
  .:::::::::::::::::::::::. xx               .::xxxx :::::::::::::::::::::::
  :::::::::::::::::::::::::.'xxx..        .::xxxxxxx ::::::::::::::::::::'
  '::::::::::::::::::::::::: xxxxxxxxxxxxxxxxxxxxxxx :::::::::::::::::'
    '::::::::::::::::::::::: xxxxxxxxxxxxxxxxxxxxxxx :::::::::::::::'
        ':::::::::::::::::::_xxxxxx::'''::xxxxxxxxxx '::::::::::::'
             '':.::::::::::'                        `._'::::::''
";
        println!("{}", tux);
    }
}
