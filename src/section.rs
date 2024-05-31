use std::fmt::Error;

use crossterm::event::KeyCode;

pub struct Section {
    pub column: u16,
    pub row: u16,
    prefix: String,
    suffix: Vec<char>,
}

impl Section {
    pub fn new(prefix: &str, column: u16, row: u16) -> Self {
        Section {
            column,
            row,
            prefix: prefix.to_owned(),
            suffix: Vec::new(),
        }
    }

    pub fn parse_input(&mut self, code: KeyCode) {
        match code {
            KeyCode::Char(c) => {
                self.suffix.push(c);
                self.column += 1;
            }
            KeyCode::Enter => {
                self.suffix.push('\r');
                self.column = 0;
                self.row += 1;
            }
            KeyCode::Backspace => {
                if !self.suffix.is_empty() {
                    let removed = self.suffix.remove(self.column as usize);
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

#[cfg(test)]
mod test {
    use super::Section;

    #[test]
    fn get_position_correctly() {
        let section = Section::new("Hello world this is a section", 20, 5);
        assert_eq!(section.column, 20);
        assert_eq!(section.row, 5);
    }
}
