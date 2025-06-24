use std::process::Command;

fn main() {
    let mv_cmd = Command::new("mv")
        .arg("final.lunpack")
        .arg("final.tar.gz")
        .status()
        .expect("failed to execute process");
    println!("process 'mv' exited with: {}", mv_cmd);

    let mv_cmd2 = Command::new("mv")
        .arg("final.tar.gz")
        .arg("tools/")
        .status()
        .expect("failed to execute process");
    println!("process 'mv' exited with: {}", mv_cmd2);

    let status = Command::new("tar")
        .arg("-xzf")
        .arg("tools/final.tar.gz")
        .status()
        .expect("failed to execute process!");
    println!("process 'gzip' exited with: {}", status);

    let status = Command::new("mv")
        .arg("tools/LunSystems/")
        .arg("../lunpack")
        .status()
        .expect("failed to execute process!");
    println!("process 'mv' exited with: {}", status);
}
