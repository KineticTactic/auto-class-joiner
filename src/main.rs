mod lib;
use crate::lib::*;

fn main() {
    println!("Starting Auto Class Joiner...");
    run();
    loop {
        std::thread::sleep(std::time::Duration::from_millis(500));
    }
}
