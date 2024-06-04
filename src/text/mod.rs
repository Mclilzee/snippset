pub mod editable_text;

#[derive(Debug, PartialEq)]
pub struct TextRange {
    cursor_position: Option<(u16, u16)>,
    start: (u16, u16),
    end: (u16, u16),
    text: String,
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
