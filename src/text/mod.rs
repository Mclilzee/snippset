pub mod editable_text;

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
