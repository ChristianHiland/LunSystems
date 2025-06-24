use std::process::Command;

pub fn make_dir_at(path: String) {
    let mut dir_cmd = Command::new("mkdir");
    dir_cmd.arg("-p");
    dir_cmd.arg(path);
    dir_cmd.output().expect("Failed to make dir.");
}

pub fn mount_disk_at(disk: String, at: String) {
    let mut mount_cmd = Command::new("mount");
    mount_cmd.arg(disk);
    mount_cmd.arg(at);
    mount_cmd.output().expect("Failed to mount.");
}

pub fn install_howling(args: Vec<String>) -> i8 {
    println!("Mounting...");
    // Making Root dir
    make_dir_at(args[3].clone());
    // Mounting root drive.
    mount_disk_at(args[2].clone(), args[3].clone());
    return 0;
}