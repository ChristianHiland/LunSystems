use std::path::PathBuf;

pub fn _debug_step_shutdown(root_path: PathBuf, debug: bool) {
    if debug {
        println!("[DEBUG] DEBUG STEP END EXIT -- SHUTDOWN");
        println!("[DEBUG] Home Path: {:?}", root_path.display());
        println!("[DEBUG] All DLLs have been Dumped and shutdown.");
        println!("[DEBUG] Ready to exit...");   
    }
    println!("Goodbye!");
    std::process::exit(0);
}