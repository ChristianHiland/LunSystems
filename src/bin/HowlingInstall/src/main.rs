use terminal_size::{Width, Height, terminal_size};
use std::env;
use install::{install_howling};

mod install;

fn main() {
    // Collect all arguments into a Vec<String>
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        eprintln!("ADD A ARGUMENT!");
    }
    
    if let Some((Width(w), Height(h))) = terminal_size() {
        if args[1] == "neofetch" {
            let half_x = w / 2;
            for i in 0..half_x {
                print!("-");
            }
            println!("Howling OS");
            println!("Version: 0.0.5");
            println!("Features: ");
            let features: Vec<&str> = vec!["(WIP) Install automation", "(WIP) fdisk support", "(WIP) GRUB Bootloader Support & Install"]; 
            for feature in features {
                println!("   - {}", feature);
            }
        }
    } else {
        println!("Unable to get term size :(");
    }

    if args[1] == "install" {
        install_howling(args.clone());
    }
    
    println!("Hello, world!");
}


