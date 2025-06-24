use std::env;
use std::io::Error;
use std::path::{Path, PathBuf};
use std::process::Command;
use libloading::Library;

pub fn wait_for_input() {
    println!("Press enter to continue...");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    if env::consts::OS == "linux" {
        let mut command = Command::new("clear");
        command.status().expect("failed to execute process");
    } else if env::consts::OS == "windows" {
        let mut command = Command::new("cls");
        command.status().expect("failed to execute process");
    }
    return;
}

pub unsafe fn load_dll(path:  &Path) -> Result<Library, Error> {
    let lib = match Library::new(path) {
        Ok(lib) => lib,
        Err(e) => {
            eprintln!("Error loading library: {}", e);
            return Ok(Library::new(path).unwrap());
        }
    };
    Ok(lib)
}

pub fn get_root_path() -> Result<PathBuf, Error> {
    let current_dir = env::current_dir();
    match current_dir {
        Ok(path) => Ok(path),
        Err(error) => Err(error.into()),
    }
}

pub fn update_lib(platform: String, filename: String, url: String) -> Result<(), Error> {
    let cmd = Command::new("python3")
        .arg("scripts/lib/fetch_lib.py")
        .arg(&platform)
        .arg(&filename)
        .arg(&url)
        .status()
        .expect("failed to execute process");
    Ok(())
}

pub fn _soft_reboot(platform: String, filename: String) {
    println!("[DEBUG] SOFT REBOOT");
    println!("[DEBUG] Rebooting...");
    eprintln!("[MAJOR ERROR] HARD REBOOT.......");
    println!("[DEBUG] DLL have been updated On next start.");
    let cmd = Command::new("python3")
        .arg("scripts/lib/fetch_lib.py")
        .arg("CONT")
        .arg(&platform)
        .arg(&filename)
        .status()
        .expect("failed to execute process");
}
