pub mod editable_text;

pub trait TerminalText {
    fn cursor(&self) -> Option<usize>;
    fn chars(&self) -> &Vec<char>;
    fn terminal_position(&self, width: u16) -> TextRange {
        let cursor = self.cursor();
        let mut row = 0;
        let mut column = 0;
        let mut text = String::with_capacity(self.chars().len());
        let mut cursor_position = None;
        for (i, c) in self.chars().iter().enumerate() {
            text.push(*c);
            if c == &'\r' || column > width {
                row += 1;
                column = 0;
            } else {
                column += 1;
            }

            if let Some(pos) = cursor {
                if pos == i {
                    cursor_position = Some((column, row));
                }
            }
        }

        TextRange::new((column, row), text, cursor_position)
    }
}

#[derive(Debug, PartialEq)]
pub struct TextRange {
    pub cursor_position: Option<(u16, u16)>,
    pub start: (u16, u16),
    pub end: (u16, u16),
    pub text: String,
}

impl TextRange {
    pub fn new(end: (u16, u16), text: String, position: Option<(u16, u16)>) -> Self {
        TextRange {
            start: (0, 0),
            cursor_position: position,
            end,
            text,
        }
    }
}
