use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
struct SatelliteStatus {
    name: String,
    last_code: String,
    timestamp: DateTime<Utc>,
}

fn inputs(mut inputname: &mut String) {
    io::stdin()
        .read_line(&mut inputname)
        .expect("An error occurred.");
}

fn main() {
    let file_path = "satellite.json";

    match fs::read_to_string(file_path) {
        Ok(content) => {
            let mut old_file: SatelliteStatus =
                serde_json::from_str(&content).expect("An error occurred during file reading.");
            println!("Please, insert the new trasmission code.");
            io::stdout().flush().unwrap();
            let mut input_1 = String::new();
            inputs(&mut input_1);
            let cleaned_input = input_1.trim();

            if cleaned_input == old_file.last_code {
                println!("Stable signal. No updates needed.");
                return;
            } else {
                println!("We`re saving the new status...");
                old_file.last_code = cleaned_input.to_string();
                old_file.timestamp = Utc::now();

                let json_old_file = serde_json::to_string_pretty(&old_file).expect("An error occurred during .JSON data saving.");
                fs::write(file_path, json_old_file).expect("An error occurred during .JSON file writing.");
            }
        }
        Err(_) => {
            println!("No file found. We`re creating another one.");
            println!("\nHow would you like to name your satellite?");
            io::stdout().flush().unwrap();
            let mut input_2 = String::new();
            inputs(&mut input_2);
            println!("\nWhat would you like the first code to be?");
            io::stdout().flush().unwrap();
            let mut input_3 = String::new();
            inputs(&mut input_3);

            let new_satellite = SatelliteStatus {
                name: input_2,
                last_code: input_3,
                timestamp: Utc::now(),
            };
            let json = serde_json::to_string_pretty(&new_satellite)
                .expect("An error occurred during the .JSON data saving.");
            fs::write(file_path, json).expect("An error occurred during .JSON file writing.");
            println!("Successfully updated your new satellite!")
        }
    }
}
