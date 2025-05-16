use std::process::Command;
// Self-Made Modules.
use math::*;
use backend::*;

mod math;
mod backend;

/// # _test
/// Use: The test func is with the '_' show that it is
/// an internal func that is exposed for out of .dll
/// fields.
/// Returns: i8(0): Done, i8(-1): Error, i8(1): Error, Continued.
#[unsafe(no_mangle)]
pub extern "C" fn _test() -> i8 {
    println!("Test");
    0
}

pub fn handle_dll_upgrading(path: String, platform: String) -> i8 {
    // Getting Path and making the full path.
    let name = "/(new)LunTool.";
    let url = "https://github.com/LunSystems/LunTool/releases/download/v1.0.0/LunTool.dll";
    let path = format!("{}/DLLs/{}{}", path, platform, name);
    let mut final_path = String::new();
    if platform == "windows" {
        final_path = format!("{}.dll", path);
    } else if platform == "linux" {
        final_path = format!("{}.so", path);
    }
    // Downloading a file
    let final_cmd = format!("curl -o {} {}", final_path, url);
    let mut command = Command::new(final_cmd);
    command.status().expect("Failed to execute command");
    0
}

/// # _update_libs
/// Use: The init func is with the '_' show that it is
/// an internal func that is exposed for out of .dll
/// fields.
/// Returns: i8(0): Done, i8(-1): Error, i8(1): Error, Continued.
#[unsafe(no_mangle)]
pub extern "C" fn _update_libs(path: String, platform: String) -> String {
    println!("Updating LunTool DLL...");
    if handle_dll_upgrading(path, platform) == 0 {
        // Return to the main program to tell it to do a SOFT REBOOT of LunSystems & Libs.
        return "SOFT REBOOT".to_string();
    } else {
        return "ERROR".to_string();
    }
}

/// # _init
/// Use: The init func is with the '_' show that it is
/// an internal func that is exposed for out of .dll
/// fields.
/// Returns: i8(0): Done, i8(-1): Error, i8(1): Error, Continued.
#[unsafe(no_mangle)]
pub extern "C" fn __init() -> i8 {
    step_1();
    println!("Init");
    0
}

/// # _start
/// Use: The start func is with the '_' show that it is
/// an internal func that is exposed for out of .dll
/// fields.
/// Returns: i8(0): Done, i8(-1): Error, i8(1): Error, Continued.
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> i8 {
    println!("Start");
    0
}

/// # _update
/// Use: The update func is with the '_' show that it is
/// an internal func that is exposed for out of .dll
/// fields.
/// Returns: i8(0): Done, i8(-1): Error, i8(1): Error, Continued.
#[unsafe(no_mangle)]
pub extern "C" fn _update() -> i8 {
    println!("Update");
    0
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_start() {
        let result = _start();
        if result != 0 && result == 1 {
            assert_eq!(result, 1);
        } else if result == -1 {
            assert_eq!(result, 0);
        }
        assert_eq!(result, 0);
    }

    #[test]
    fn test_update() {
        let result = _update();
        if result != 0 && result == 1 {
            assert_eq!(result, 1);
        } else if result == -1 {
            assert_eq!(result, 0);
        }
        assert_eq!(result, 0);
    }
}
