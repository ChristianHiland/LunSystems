// src/main.rs

fn main() {
    println!("Welcome to LunSystems!");

    // This block is only compiled if `CONFIG_ENABLE_FEATURE_X=y` was set
    #[cfg(howling_install_compile)] { println!("HOWLING_INSTALL_COMPILE is enabled!"); }
}