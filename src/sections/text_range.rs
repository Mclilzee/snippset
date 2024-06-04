use super::section::Section;

pub struct TextRange {
    cursor_position: Option<(u16, u16)>,
    start: (u16, u16),
    end: (u16, u16),
}

impl TextRange {
    pub fn new(start: (u16, u16), end: (u16, u16), position: Option<(u16, u16)>) -> Self {
        Self {
            cursor_position: position,
            start,
            end,
        }
    }
}
