use std::fs::OpenOptions;
use std::io::{Write, Result};
use chrono::prelude::*;

pub struct FileWriter {
    file: std::fs::File,
}

impl FileWriter {
    pub fn new(file_path: &str) -> Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;

        Ok(Self { file })
    }

    pub fn log_word(&mut self, word: &str) -> Result<()> {
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry = format!("[{}] Word: {}\n", timestamp, word);
        self.file.write_all(log_entry.as_bytes())?;
        self.file.flush()?;
        Ok(())
    }

    pub fn log_special_key(&mut self, special_key: &str) -> Result<()> {
        let timestamp = Utc::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry = format!("[{}] Special: {}\n", timestamp, special_key);
        self.file.write_all(log_entry.as_bytes())?;
        self.file.flush()?;
        Ok(())
    }
}