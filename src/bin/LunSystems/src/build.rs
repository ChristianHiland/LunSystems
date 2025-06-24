// build.rs
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let config_path = Path::new(".config");

    // Tell Cargo to rerun this script if .config changes.
    println!("cargo:rerun-if-changed=.config");

    // Do nothing if .config doesn't exist.
    // This allows `cargo build` to work before the first `make menuconfig`.
    if !config_path.exists() {
        return;
    }

    let file = File::open(config_path).expect(".config file not found");
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line.expect("Failed to read line from .config");

        // Ignore comments and empty lines
        if line.trim().is_empty() || line.starts_with('#') {
            continue;
        }

        // Split the line into key and value
        if let Some((key, value)) = line.split_once('=') {
            // Trim whitespace and the "CONFIG_" prefix from the key
            let key = key.trim();
            if let Some(cfg_key) = key.strip_prefix("CONFIG_") {
                let value = value.trim();

                // For boolean 'y', set a cfg flag.
                // We convert the key to lowercase as is conventional for cfg flags.
                if value == "y" {
                    println!("cargo:rustc-cfg={}", cfg_key.to_lowercase());
                }
                // For other values (strings, numbers), set an environment variable.
                // We keep the original CONFIG_ prefix for clarity.
                else {
                    // Remove quotes from string values
                    let value = value.trim_matches('"');
                    println!("cargo:rustc-env={}={}", key, value);
                }
            }
        }
    }
}