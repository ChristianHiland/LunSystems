use std::path::{Path, PathBuf};
use std::process::Command;
use libloading::Library;
use std::io::Error;
use std::env;

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

pub fn run_shell_script(filepath: String, count: i8) {
    let mut sh_cmd = Command::new("./");
    sh_cmd.arg(filepath);
    sh_cmd.arg(format!("-{}", count));
    sh_cmd.output().expect("Failed to run shell script.");
}

pub fn make_dir_at(path: String) {
    let mut dir_cmd = Command::new("mkdir");
    dir_cmd.arg("-p");
    dir_cmd.arg(path);
    dir_cmd.output().expect("Failed to make dir.");
}

pub fn run_fdisk() {
    let mut fdisk_cmd = Command::new("fdisk");
    fdisk_cmd.arg("-l");
    fdisk_cmd.output().expect("Failed to run fdisk.");
}

pub fn copy_file_to(filepath:String, path: String) {
    let mut cp_cmd = Command::new("cp");
    cp_cmd.arg(filepath);
    cp_cmd.arg(path);
    cp_cmd.output().expect("Failed to copy");
}

pub fn list_dir(path: String) {
    let mut ls_cmd = Command::new("ls");
    ls_cmd.output().expect("Failed to list.");
}

pub fn list_dir_table(path: String, count: i8) {
    let mut ls_cmd = Command::new("ls");
    ls_cmd.arg(format!("-{}", count));
    ls_cmd.arg("-l");
    ls_cmd.output().expect("Failed to list.");
}

pub fn mount_disk_at_uuid(uuid: String, at: String) {
    let mut mount_cmd = Command::new("mount");
    mount_cmd.arg("--uuid");
    mount_cmd.arg(uuid);
    mount_cmd.arg(at);
    mount_cmd.output().expect("Failed to mount.");
}

pub fn install_package_network(args: Vec<String>) {

}