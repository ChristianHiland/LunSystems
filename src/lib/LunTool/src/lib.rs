use std::time::{ Duration, Instant };
use std::process::Command;
use std::thread::sleep;
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
    println!("Testing...");

    sleep(Duration::new(3,0));
    println!("[TEST] Testing init...");
    if (__init() == 0) {
        println!("[TEST] Init test passed!");
    } else { println!("[TEST] Init test failed!")}
    
    sleep(Duration::new(3,0));
    println!("[TEST] Testing start...");
    if (_start() == 0) {
        println!("[TEST] start test passed!");
    } else { println!("[TEST] start test failed!")}
    
    sleep(Duration::new(3,0));
    println!("[TEST] Testing update...");
    if (_update() == 0) {
        println!("[TEST] update test passed!");
    } else { println!("[TEST] update test failed!")}
    0
}

pub fn handle_dll_upgrading(path: String, platform: String) -> i8 {
    // Getting Path and making the full path.
    let name = format!("{}/libLunTool", path);
    println!("Platform: {}", platform);
    let url = "https://github.com/ChristianHiland/LunTool/releases/download/Test/libLunTool";
    let path = format!("{}/DLLs/{}{}", path, platform, name);
    let mut final_path = String::new();
    let mut filename = "woof".to_string();
    let mut final_url  = String::new();
    if platform == "windows" {
        let url2 = url;
        filename = format!("{}.dll", name);
        final_url = format!("{}.dll", url2);
        final_path = format!("{}.dll", path);
    } else if platform == "linux" {
        let url2 = url;
        filename = format!("{}.so", name);
        final_url = format!("{}.so", url2);
        final_path = format!("{}.so", path);
    }
    // Downloading a file
    let status = Command::new("curl")
        .arg("-o")
        .arg(filename)
        .arg(final_url)
        .status();
    match status {
        Ok(result) => {
            println!("{}", result);
        }
        
        Err(e) => {
            println!("{}", e);
        }
    }
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
