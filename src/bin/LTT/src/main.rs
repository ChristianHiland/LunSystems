mod steps;
mod helpers;

use libloading::{Library, Symbol};
use std::path::{Path, PathBuf};
use helpers::{ wait_for_input, load_dll, get_root_path, _soft_reboot, update_lib};
use std::process::Command;
use steps::_debug_step_shutdown;
use std::io::Error;
use std::{env, fs};

fn main() {
    let mut should_not_quit = true;
    let mut debug = false;
    let mut input = String::new();
    wait_for_input();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    if input == "y\n" || input == "Y\n" {
        debug = true;
    } else {
        debug = false;
    }
    
    
    if debug { println!("[DEBUG] DEBUG ENABLED\n"); }

    // Loading in DLLs
    if debug { println!("[DEBUG] LOADING DLLs..."); }
    if debug { println!("[DEBUG] LOADING LunTools.dll..."); }

    // Loading LunTool DLL file by getting the current root path and then getting the dll.
    let root_path = get_root_path().unwrap();
    let lun_tool_dll: Library;
    if env::consts::OS == "linux" {
        if fs::exists(format!("{}/lib/linux/libLunTool.so", root_path.display())).expect("REASON") {
            let path = format!("{}/lib/linux/libLunTool.so", root_path.display());
            let luntool_dll = Path::new(path.as_str());
            lun_tool_dll = unsafe { load_dll(luntool_dll).expect("TODO: panic message") };
        } else {
            println!("[ERROR] LunTool.dll not found. Please run the update script.");
            return;
        }
    } else if env::consts::OS == "windows" {
        if fs::exists(format!("{}/lib/windows/libLunTool.dll", root_path.display())).expect("REASON") {
            let path = format!("{}/lib/windows/libLunTool.dll", root_path.display());
            let luntool_dll = Path::new(path.as_str());
            lun_tool_dll = unsafe { load_dll(luntool_dll).expect("TODO: panic message") };
        } else {
            println!("[ERROR] LunTool.dll not found. Please run the update script.");
            return;
        }
    } else {
        if fs::exists("/lib/howling/libLunTool.so".to_string()).expect("REASON") {
            let path = "/lib/howling/libLunTool.so".to_string();
            let luntool_dll = Path::new(path.as_str());
            lun_tool_dll = unsafe { load_dll(luntool_dll).expect("TODO: panic message") };
        } else {
            println!("[ERROR] LunTool.dll not found. Please run the update script.");
            return;
        }
    }

    
    unsafe {
        let platform = env::consts::OS.to_string();
        let luntool_start: Symbol<unsafe extern   "C" fn()    -> i64> =
            match lun_tool_dll.get(b"_start\0") { Ok(retu) => { retu } Err(e) => { println!("[ERROR] LunTool.dll init failed: {}", e); return;} };
        let luntool_test: Symbol<unsafe extern    "C" fn()    -> i64> =
            match lun_tool_dll.get(b"_test\0") { Ok(retu) => { retu } Err(e) => { println!("[ERROR] LunTool.dll init failed: {}", e); return;} };
        let luntool_update:  Symbol<unsafe extern "C" fn(path: String, platform: String)    -> String> =
            match lun_tool_dll.get(b"_update_libs\0") { Ok(retu) => { retu } Err(e) => { println!("[ERROR] LunTool.dll init failed: {}", e); return;} };
        let luntool_init:    Symbol<unsafe extern "C" fn()    -> i64> =
            match lun_tool_dll.get(b"__init\0") { Ok(retu) => { retu } Err(e) => { println!("[ERROR] LunTool.dll init failed: {}", e); return;} };
        let luntool_version: Symbol<unsafe extern "C" fn(path: String)    -> i64> =
            match lun_tool_dll.get(b"_version\0") { Ok(retu) => { retu } Err(e) => { println!("[ERROR] LunTool.dll init failed: {}", e); return;}  };
        let luntool_unload:  Symbol<unsafe extern "C" fn()    -> i64> =
            match lun_tool_dll.get(b"_unload\0") { Ok(retu) => { retu } Err(e) => { println!("[ERROR] LunTool.dll init failed: {}", e); return;} };
        //luntool_init();


        let mut ran_once: bool = false;
        while (should_not_quit) {
            if !ran_once {
                println!("LTT is now running...");
                println!("Welcome thanks for using LTT! Please input the number of the function you would like to use.");
                ran_once = true;
            } else {
                if env::consts::OS == "linux" {
                    let mut command = Command::new("clear");
                    command.status().expect("failed to execute process");
                } else if env::consts::OS == "windows" {
                    let mut command = Command::new("cls");
                    command.status().expect("failed to execute process");
                }

            }
            println!("1. Start");
            println!("2. Test LunDLLs");
            println!("3. Update DLLs");
            println!("4. Install LTT (LINUX)");
            println!("6. Version");
            println!("7. Unload FUNCS IN DLLs");
            println!("8. Exit");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).expect("Failed to read line");
            let input_num: i64 = input.trim().parse().unwrap();
            if input_num == 1 {
                luntool_start();
                wait_for_input();
            } else if input_num == 2 {
                luntool_test();
                wait_for_input();
            } else if input_num == 3 {
                let mut filename = String::new();
                let mut url = String::new();
                if platform == "linux" {
                    url = "https://github.com/ChristianHiland/LunSystems/raw/refs/heads/master/target/release/libLunTool.so".to_string();
                    filename = format!("libLunTool.so");
                } else if platform == "windows" {
                    url = "https://github.com/ChristianHiland/LunSystems/raw/refs/heads/master/target/release/libLunTool.dll".to_string();
                    filename = format!("libLunTool.dll");
                }
                update_lib(platform.clone().to_string(), filename.to_string(), url);
                if "SOFT REBOOT" == "SOFT REBOOT".to_string() { _soft_reboot(platform.clone(), filename); }
                should_not_quit = false;
            } else if input_num == 4 {
                let cmd = Command::new("python3")
                    .arg("Install/Scripts/Install_Linux.py")
                    .arg(root_path.display().to_string())
                    .status()
                    .expect("failed to execute process");
            } else if input_num == 5 {
                let cmd = Command::new("python3")
                    .arg("Install/Scripts/Install_Windows.py")
                    .status()
                    .expect("failed to execute process");
            } else if input_num == 6 {
                println!("LTT Version 0.1.4");
                luntool_version(root_path.display().to_string());
                wait_for_input();
            } else if input_num == 7 {
                luntool_unload();    
            } else if input_num == 8 {
                should_not_quit = false;
            } else {
                println!("Invalid input. Please try again.");
            }
        }
        luntool_unload();
        let mut value = drop(lun_tool_dll);
    }

    // End of Step One. (Loading out from init phase into Ready.)
    if debug { _debug_step_shutdown(root_path, debug); }
    
    return;
}
