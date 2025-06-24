use std::process::Command;

fn main() {
    let mv_cmd = Command::new("mv")
        .arg("final.lunpack")
        .arg("final.cpio.gz")
        .status()
        .expect("failed to execute process");
    println!("process 'cp' exited with: {}", mv_cmd);

    let mv_cmd2 = Command::new("mv")
        .arg("final.cpio.gz")
        .arg("tools/")
        .status()
        .expect("failed to execute process");
    println!("process 'mv' exited with: {}", mv_cmd2);

    let status = Command::new("./tools/gzip")
        .arg("-dv")
        .arg("final.cpio.gz")
        .status()
        .expect("failed to execute process!");
    println!("process 'mv' exited with: {}", status);
}
