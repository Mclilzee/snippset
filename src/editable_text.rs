use crossterm::event::KeyCode;

pub struct EditableText {
    index: usize,
    chars: Vec<char>,
}

impl EditableText {
    pub fn new() -> Self {
        EditableText {
            index: 0,
            chars: Vec::new(),
        }
    }

    pub fn parse_input(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char(c) => {
                self.chars.push(c);
                self.index += 1;
            }
            KeyCode::Enter => {
                self.chars.push('\r');
                self.index += 1;
            }
            KeyCode::Backspace => {
                if !self.chars.is_empty() {
                    self.chars.remove(self.index);
                    self.index -= 1;
                }
            }
            _ => (),
        };
    }
}
