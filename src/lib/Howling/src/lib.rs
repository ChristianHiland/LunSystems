// Import Modules
use serde::{Deserialize, Serialize}; // Import the necessary traits
use std::io::{self, Write};
use funcs::{*};
use data::{*};
use std::fs;                       // For file system operations



// My Modules
mod funcs;
mod data;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[unsafe(no_mangle)]
extern "C" fn test() -> i8 {
    println!("Testing Howling Lib!...");
    return 0;
}

#[unsafe(no_mangle)]
extern "C" fn _check_os(os_config: String) -> Vec<String> {
    let root_path = crate::get_root_path().unwrap();
    let full_path = root_path.join(os_config);
    vec![full_path.to_str().unwrap().to_string()]
}

#[unsafe(no_mangle)]
extern "C" fn _find_updates(packages_file: String) {
    
}

#[unsafe(no_mangle)]
extern "C" fn __init(os_config: String) -> bool {
    // Setting up config.
    let full_path = "/Howling/os.json";

    println!("\n--- Writing to JSON file ---");
    let config_to_write = HowlingOsConfig {
        co_name: "Pup".to_string(),
        version: "0.0.6".to_string(),
        packages: 1000,
    };

    let result = make_os_config(full_path.to_string(), config_to_write).unwrap();
    if result == false {
        println!("Making Config file failed!");
        return false;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
