use crate::funcs::{get_root_path, load_dll};
use libloading::{Error, Library};
use std::path::Path;
use std::{env, fs};


pub unsafe fn _load_dll(filename: String) -> Library {
    // Loading LunTool DLL file by getting the current root path and then getting the dll.
    let root_path = get_root_path().unwrap();
    let lun_tool_dll: Library;

    if env::consts::OS == "linux" {
        if fs::exists(format!("/lib/Howling/{}", filename)).expect("REASON") {
            let path = format!("{}/lib/linux/libLunTool.so", root_path.display());
            let luntool_dll = Path::new(path.as_str());
            lun_tool_dll = unsafe { load_dll(luntool_dll).expect("TODO: panic message")};
            return lun_tool_dll;
        } else {
            println!("[ERROR] LunTool.dll not found. Please run the update script.");
            return Library::new("Non").unwrap();
        }
    } else if env::consts::OS == "windows" {
        if fs::exists(format!("{}/lib/windows/{}", root_path.display(), filename)).expect("REASON") {
            let path = format!("{}/lib/windows/libLunTool.dll", root_path.display());
            let luntool_dll = Path::new(path.as_str());
            lun_tool_dll = unsafe { load_dll(luntool_dll).expect("TODO: panic message") };
            return lun_tool_dll;
        } else {
            println!("[ERROR] LunTool.dll not found. Please run the update script.");
            return Library::new("non").unwrap();
        }
    } else {
        if fs::exists("/lib/howling/libLunTool.so".to_string()).expect("REASON") {
            let path = "/lib/howling/libLunTool.so".to_string();
            let luntool_dll = Path::new(path.as_str());
            lun_tool_dll = unsafe { load_dll(luntool_dll).expect("TODO: panic message") };
            return lun_tool_dll
        } else {
            println!("[ERROR] LunTool.dll not found. Please run the update script.");
            return Library::new("non").unwrap();
        }
    }

}