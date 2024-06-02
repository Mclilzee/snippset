use super::section::Section;

pub struct TextRange {
    cursor_position: (u16, u16),
    edit_start: (u16, u16),
    edit_end: (u16, u16),
    text_end: (u16, u16),
}

impl TextRange {
    pub fn new(sections: &[Section]) -> Self {}
}
