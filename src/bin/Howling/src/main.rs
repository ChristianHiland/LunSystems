use crate::funcs::{get_root_path, wait_for_input, list_dir, copy_file_to, run_fdisk, run_shell_script, install_package_network};
use libloading::{Library, Symbol};
use helper::{_load_dll};
use managers::{*};
use std::env;

mod json_manager;
pub mod helper;
pub mod funcs;
mod managers;
mod structs;


pub fn debug_enable() -> bool {
    let mut debug = false;
    let mut input = String::new();
    wait_for_input();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    if input == "y\n" || input == "Y\n" { debug = true; } else { debug = false; }
    debug
}


fn main() {
    let root_path = get_root_path().unwrap();

    // Debug
    let mut should_not_quit = true;
    let debug = debug_enable();
    if debug { println!("[DEBUG] DEBUG ENABLED\n"); }

    // Loading in DLLs
    if debug { println!("[DEBUG] LOADING DLLs..."); }

    if debug { println!("[DEBUG] LOADING LibHowling.so..."); }
    let howling_lib = unsafe { _load_dll("libHowling.so".to_string()) };

    if debug { println!("[DEBUG] LOADING LibLunTool.so..."); }
    let luntool_lib = unsafe { _load_dll("libLunTool.so".to_string()) };

    // Getting Functions...
    if debug { println!("[DEBUG] LOADING DLL Functions..."); }

    // LunTools
    let lun_tool_init: Symbol<unsafe extern   "C" fn()    -> i64> =
        match unsafe { luntool_lib.get(b"_init\0") } { Ok(retu) => { retu } Err(e) => { println!("[ERROR] LunTool.dll init failed: {}", e); return;} };

    if debug { println!("[DEBUG] Init DLLs..."); }
    // Init Dlls...
    unsafe {
        if lun_tool_init() != 0 { 
            println!("[ERROR] LunTool.dll INIT FAILED!");
            return;
        }
    }

    // MAIN HOWLING PROGRAM
    // Collect all arguments into a Vec<String>
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 1 {
        println!("ARG NEEDED");
        return;
    }

    // Manage Updating HowlingOS (Update like in APT.)
    if args[1] == "update" {

    }

    // Manage Install HowlingOS (Install like in APT.)
    if args[1] == "install" {
        // For when networking works.
        if args.len() < 2 {
            if args[2] == "--network" {
                // Network Function.
                install_package_network(args.clone());
            }
        }
        let root_path = format!("/Howling/Packages/Drive/bins/{}", args[2]);
        copy_file_to(root_path, "/bin/".to_string());
    }

    if args[1] == "internet" {
        if args[2] == "list" {
        } else if args[2] == "connect" {

        }
    }

    if args[1] == "mount" {
        if args[2] == "packages" {
            mount_packages_boot("a57beb0f-7a26-4a68-adb5-ee982b12ab62".to_string(), "/Howling/Packages/Drive".to_string());
        }
    }

    if args[1] == "packages" {
        list_dir("/Howling/Packages/Drives/bins/".to_string());
    }

    // Manage Listing HowlingOS (List like in APT.)
    if args[1] == "list" {
        if args[2] == "drives" {
            run_shell_script("/bin/ListDrives".to_string(), 15);
        }
        list_packages("/Howling/Packages/list.json".to_string());
    }

    // Manage init HowlingOS.
    if args[1] == "init" {
        init_package_manager("/Howling/Packages/list.json".to_string());
    }
}
