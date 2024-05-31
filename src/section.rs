use crossterm::event::KeyCode;

pub struct Section {
    pub position: (u16, u16),
    text: Vec<char>,
}

impl Section {
    pub fn new(column: u16, row: u16) -> Self {
        Section {
            position: (column, row),
            text: Vec::new(),
        }
    }

    pub fn parse_input(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char(c) => {
                self.text.push(c);
                self.position.1 += 1;
            }
            KeyCode::Enter => {
                self.text.push('\r');
                self.position.0 = 0;
                self.position.1 += 1;
            }
            KeyCode::Backspace => {
                let removed = self.text.remove(self.position.0 as usize);
                self.position.0 -= 1;
                if removed == '\r' {
                    self.position.1 -= 1;
                };
            }
            _ => (),
        };
    }
}
