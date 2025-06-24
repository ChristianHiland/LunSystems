use std::process::Command;

fn main() {
    let mv_cmd = Command::new("mv")
        .arg("final.lunpack")
        .arg("final.cpio.gz")
        .status()
        .expect("failed to execute process");
    println!("process 'mv' exited with: {}", mv_cmd);

    let mv_cmd2 = Command::new("mv")
        .arg("final.cpio.gz")
        .arg("tools/")
        .status()
        .expect("failed to execute process");
    println!("process 'mv' exited with: {}", mv_cmd2);

    let status = Command::new("./tools/gzip")
        .arg("-dv")
        .arg("tools/final.cpio.gz")
        .status()
        .expect("failed to execute process!");
    println!("process 'gzip' exited with: {}", status);
    let status = Command::new("cpio")
        .arg("-id")
        .arg("<")
        .arg("tools/final.cpio")
        .status()
        .expect("failed to execute process!");
    println!("process 'cpio' exited with: {}", status);
}
