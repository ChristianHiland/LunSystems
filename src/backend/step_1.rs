use std::time::{ Duration, Instant };
use std::thread::sleep;

pub fn step_1() {
    println!("[DLL - Step 1] Step 1 Init...");
    for i in 0..200 {
        sleep(Duration::new(2,0));
        print!(".");
    }
    sleep(Duration::new(3,0));
    println!("[DLL - Step 1] Step 1 Done!");
}



#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_step_1() {
        step_1();
    }
}