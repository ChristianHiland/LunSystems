// Importing Needed Modules
use std::fs::File;
use std::io::Write;
use std::io::Read;
use serde_json;
use crate::structs::{*};

#[warn(dead_code)]
pub fn read_json_howling_packs(filename: String) -> HowlingPacks {
    let mut file = match File::open(&filename) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Error Opening File '{}': {}", &filename, e);
            return HowlingPacks::new();
        }
    };

    // Reading JSON File into contents
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        eprintln!("Error Reading File '{}': {}", filename, e);
        return HowlingPacks::new();
    }

    // Deserializing.
    let mut file_content = HowlingPacks::new();
    match serde_json::from_str::<HowlingPacks>(&contents) {
        Ok(content) => {
            println!("Successfully deserialized: {:?}", content);
            println!("Version: {}", content.version);
            file_content = content;
        }

        Err(e) => {
            eprintln!("Error Deserialing JSON: {}", e);
        }
    }
    return file_content;
}

pub fn write_json_howling_packs(filename: String, data: HowlingPacks) {
    match serde_json::to_string_pretty(&data) {
        Ok(json_string) => {
            match File::create(&filename) {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(json_string.as_bytes()) {
                        eprintln!("Error writing to file '{}': {}", filename, e)
                    }
                    // Wrote TO FILE!
                }
                Err(e) => {
                    eprintln!("Error creating the file '{}': {}", filename, e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error serializing to JSON: {}", e);
        }
    }
}