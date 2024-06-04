pub mod editable_text;

pub struct TextRange {
    cursor_position: Option<(u16, u16)>,
    start: (u16, u16),
    end: (u16, u16),
    text: String,
}
