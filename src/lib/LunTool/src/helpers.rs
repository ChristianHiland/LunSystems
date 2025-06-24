// Importing Needed Modules
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::fs::File;
use std::io::Read;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct InitConfig {
    pub version:          String,
    pub config_settings:  ConfigSetting,
}

impl InitConfig {
    pub fn new() -> Self {
        Self {
            version: "",
            config_settings: ConfigSetting::new(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum SettingType {
    Bool,
    I8,
    I16,
    String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigSetting {
    pub name:            String,
    pub var_type:        i8,
    pub string_type:     String,
    pub int8:            i8,
    pub int16:           i16,
}

impl ConfigSetting {
    pub fn new() -> Self {
        Self {
            name:        "",
            var_type:    0,
            string_type: "",
            int8:        0,
            int16:       0,
        }
    }
}


pub fn read_json_init_config(filename: String) -> InitConfig {
    let mut file = match File::open(&filename) {
        Ok(file) => file,
        Err(e)   => {
            eprintln!("Error Opening File '{}': {}", &filename, e);
            return InitConfig::new();
        }
    };

    // reading Content.
    let mut contents = String::new();
    if let Err(e) = file.read_to_string(&mut contents) {
        eprintln!("Error Reading '{}': {}", filename, e);
        return InitConfig::new();
    }

    // Deserializing
    let mut file_content = InitConfig::new();
    match serde_json::from_str::<InitConfig>(&contents) {
        Ok(content) => { file_content = content; }
        Err(e)      => { eprintln!("Error Deserialing JSON: {}", e); }
    }
    return file_content;
}

pub fn write_json_init_config(filename: String, data: InitConfig) {
    match serde_json::to_string_pretty(&data) {
        Ok(json_string) => {
            match File::create(&filename) {
                Ok(mut file) => {
                    if let Err(e) = file.write_all(json_string.as_bytes()) { eprintln!("Error writing to file '{}': {}", filename, e) }
                }
                Err(e) => { eprintln!("Error creating the file '{}': {}", filename, e); }
            }
        }
        Err(e) => { eprintln!("Error serializing to JSON: {}", e); }
    }
}