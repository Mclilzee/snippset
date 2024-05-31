use crossterm::event::KeyCode;

pub struct Section {
    pub position: (u16, u16),
    prefix: String,
    suffix: Vec<char>,
}

impl Section {
    pub fn new(content: &str) -> Self {
        Section {
            position: (content.len() as u16 + 1, 0),
            prefix: content.to_owned(),
            suffix: Vec::new(),
        }
    }

    pub fn parse_input(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char(c) => {
                self.suffix.push(c);
                self.position.1 += 1;
            }
            KeyCode::Enter => {
                self.suffix.push('\r');
                self.position.0 = 0;
                self.position.1 += 1;
            }
            KeyCode::Backspace => {
                if !self.suffix.is_empty() {
                    let removed = self.suffix.remove(self.position.0 as usize);
                    self.position.0 -= 1;
                    if removed == '\r' {
                        self.position.1 -= 1;
                    };
                }
            }
            _ => (),
        };
    }
}
