use crossterm::event::KeyCode;

pub struct EditableText {
    pub column: u16,
    pub row: u16,
    chars: Vec<char>,
}

impl EditableText {
    pub fn new(column: u16, row: u16) -> Self {
        EditableText {
            column,
            row,
            chars: Vec::new(),
        }
    }

    pub fn parse_input(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char(c) => {
                self.chars.push(c);
                self.column += 1;
            }
            KeyCode::Enter => {
                self.chars.push('\r');
                self.column = 0;
                self.row += 1;
            }
            KeyCode::Backspace => {
                if !self.chars.is_empty() {
                    let removed = self.chars.remove(self.column as usize);
                    self.column -= 1;
                    if removed == '\r' {
                        self.row -= 1;
                    };
                }
            }
            _ => (),
        };
    }
}
