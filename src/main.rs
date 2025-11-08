use std::fs::{OpenOptions, File};
use std::io::{self, BufRead, BufReader, Write};

// Minimal CSV key-value store (Rust).
// Data is stored as lines "key,value" in ./data.csv.
// No external libraries, just basics.

fn main() {
    println!("CSV Key-Value Store (Rust)");
    println!("Commands: set <key> <value> | get <key> | delete <key> | list | quit\n");

    loop {
        // Prompt
        print!("> ");
        io::stdout().flush().unwrap();

        // Read command from user
        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            continue;
        }
        let line = input.trim();
        if line.is_empty() { continue; }

        let words: Vec<&str> = line.split_whitespace().collect();
        if words.is_empty() { continue; }

        // Command parsing
        match words[0] {
            "set" => {
                // Usage: set <key> <value>
                if words.len() < 3 {
                    println!("Usage: set <key> <value>");
                    continue;
                }
                let key = words[1].to_string();
                let value = words[2..].join(" "); // Combine words (to keep spaces)

                // Load all entries, update/add new, save all back
                let mut pairs = load_csv();
                // Remove previous value if it existed
                pairs.retain(|(k, _)| k != &key);
                pairs.push((key.clone(), value.clone()));

                if save_csv(&pairs).is_ok() {
                    println!("Set '{}'", key);
                } else {
                    println!("Could not update the file.");
                }
            }
            "get" => {
                // Usage: get <key>
                if words.len() < 2 {
                    println!("Usage: get <key>");
                    continue;
                }
                let key = words[1];
                let pairs = load_csv();
                let value = pairs.iter().find_map(|(k, v)| if k == key { Some(v) } else { None });
                match value {
                    Some(v) => println!("{}", v),
                    None => println!("Key not found"),
                }
            }
            "delete" => {
                // Usage: delete <key>
                if words.len() < 2 {
                    println!("Usage: delete <key>");
                    continue;
                }
                let key = words[1];
                let mut pairs = load_csv();
                let before = pairs.len();
                pairs.retain(|(k, _)| k != key);
                if pairs.len() == before {
                    println!("Key not found");
                } else {
                    if save_csv(&pairs).is_ok() {
                        println!("Deleted '{}'", key);
                    } else {
                        println!("Could not update the file.");
                    }
                }
            }
            "list" => {
                // List all pairs present
                let pairs = load_csv();
                if pairs.is_empty() {
                    println!("(empty)");
                } else {
                    for (k, v) in pairs {
                        println!("{},{}", k, v);
                    }
                }
            }
            "quit" | "exit" => {
                println!("Exiting.");
                break;
            }
            _ => {
                println!("Unknown command: {}", words[0]);
                println!("Available: set, get, delete, list, quit");
            }
        }
    }
}

// Reads all key-value pairs from data.csv into a Vec<(String, String)>
fn load_csv() -> Vec<(String, String)> {
    let mut pairs = Vec::new();
    let file = File::open("data.csv");
    if let Ok(file) = file {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(l) = line {
                let l = l.trim();
                if l.is_empty() { continue; }
                let parts: Vec<&str> = l.splitn(2, ',').collect();
                if parts.len() == 2 {
                    pairs.push((parts[0].to_string(), parts[1].to_string()));
                }
            }
        }
    }
    pairs
}

// Writes all key-value pairs to data.csv, overwrites the file
fn save_csv(pairs: &Vec<(String, String)>) -> io::Result<()> {
    let file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("data.csv")?;
    let mut writer = io::BufWriter::new(file);
    for (k, v) in pairs {
        writeln!(writer, "{},{}", k, v)?;
    }
    Ok(())
}
