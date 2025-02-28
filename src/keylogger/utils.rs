use device_query::Keycode;

pub fn keycode_to_char(key: &Keycode, keys: &[Keycode]) -> Option<char> {
    let is_shift_pressed = keys.contains(&Keycode::LShift) || keys.contains(&Keycode::RShift);

    match key {
        Keycode::A => Some(if is_shift_pressed { 'A' } else { 'a' }),
        Keycode::B => Some(if is_shift_pressed { 'B' } else { 'b' }),
        Keycode::C => Some(if is_shift_pressed { 'C' } else { 'c' }),
        Keycode::D => Some(if is_shift_pressed { 'D' } else { 'd' }),
        Keycode::E => Some(if is_shift_pressed { 'E' } else { 'e' }),
        Keycode::F => Some(if is_shift_pressed { 'F' } else { 'f' }),
        Keycode::G => Some(if is_shift_pressed { 'G' } else { 'g' }),
        Keycode::H => Some(if is_shift_pressed { 'H' } else { 'h' }),
        Keycode::I => Some(if is_shift_pressed { 'I' } else { 'i' }),
        Keycode::J => Some(if is_shift_pressed { 'J' } else { 'j' }),
        Keycode::K => Some(if is_shift_pressed { 'K' } else { 'k' }),
        Keycode::L => Some(if is_shift_pressed { 'L' } else { 'l' }),
        Keycode::M => Some(if is_shift_pressed { 'M' } else { 'm' }),
        Keycode::N => Some(if is_shift_pressed { 'N' } else { 'n' }),
        Keycode::O => Some(if is_shift_pressed { 'O' } else { 'o' }),
        Keycode::P => Some(if is_shift_pressed { 'P' } else { 'p' }),
        Keycode::Q => Some(if is_shift_pressed { 'Q' } else { 'q' }),
        Keycode::R => Some(if is_shift_pressed { 'R' } else { 'r' }),
        Keycode::S => Some(if is_shift_pressed { 'S' } else { 's' }),
        Keycode::T => Some(if is_shift_pressed { 'T' } else { 't' }),
        Keycode::U => Some(if is_shift_pressed { 'U' } else { 'u' }),
        Keycode::V => Some(if is_shift_pressed { 'V' } else { 'v' }),
        Keycode::W => Some(if is_shift_pressed { 'W' } else { 'w' }),
        Keycode::X => Some(if is_shift_pressed { 'X' } else { 'x' }),
        Keycode::Y => Some(if is_shift_pressed { 'Y' } else { 'y' }),
        Keycode::Z => Some(if is_shift_pressed { 'Z' } else { 'z' }),
        Keycode::Key0 => Some(if is_shift_pressed { ')' } else { '0' }),
        Keycode::Key1 => Some(if is_shift_pressed { '!' } else { '1' }),
        Keycode::Key2 => Some(if is_shift_pressed { '@' } else { '2' }),
        Keycode::Key3 => Some(if is_shift_pressed { '#' } else { '3' }),
        Keycode::Key4 => Some(if is_shift_pressed { '$' } else { '4' }),
        Keycode::Key5 => Some(if is_shift_pressed { '%' } else { '5' }),
        Keycode::Key6 => Some(if is_shift_pressed { '^' } else { '6' }),
        Keycode::Key7 => Some(if is_shift_pressed { '&' } else { '7' }),
        Keycode::Key8 => Some(if is_shift_pressed { '*' } else { '8' }),
        Keycode::Key9 => Some(if is_shift_pressed { '(' } else { '9' }),
        Keycode::Dot => Some(if is_shift_pressed { '>' } else { '.' }),
        Keycode::Comma => Some(if is_shift_pressed { '<' } else { ',' }),
        Keycode::Semicolon => Some(if is_shift_pressed { ':' } else { ';' }),
        Keycode::Apostrophe => Some(if is_shift_pressed { '"' } else { '\'' }),
        Keycode::Slash => Some(if is_shift_pressed { '?' } else { '/' }),
        Keycode::BackSlash => Some(if is_shift_pressed { '|' } else { '\\' }),
        Keycode::Minus => Some(if is_shift_pressed { '_' } else { '-' }),
        Keycode::Equal => Some(if is_shift_pressed { '+' } else { '=' }),
        Keycode::LeftBracket => Some(if is_shift_pressed { '{' } else { '[' }),
        Keycode::RightBracket => Some(if is_shift_pressed { '}' } else { ']' }),
        Keycode::Grave => Some(if is_shift_pressed { '~' } else { '`' }),
        _ => None,
    }
}

pub fn is_word_terminator(key: &Keycode) -> bool {
    key == &Keycode::Space ||
    key == &Keycode::Enter ||
    key == &Keycode::Tab ||
    key == &Keycode::Escape
}