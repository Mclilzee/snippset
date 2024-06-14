pub mod editable_text;
pub mod static_text;

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

#[cfg(test)]
mod test {

    #[test]
    fn get_cursor_position() {
        let editable = create_editable("ano ther");
        let text_range = editable.text_range();
        assert_eq!((14, 0), text_range.cursor_position.unwrap());
    }

    #[test]
    fn get_cursor_after_moving() {
        let mut editable = create_editable("ano ther");
        editable.move_left();
        editable.move_left();
        editable.move_left();
        assert_eq!((11, 0), editable.text_range().cursor_position.unwrap());
    }

    #[test]
    fn get_cursor_after_moving_forward() {
        let mut editable = create_editable("ano ther");
        editable.move_left();
        editable.move_left();
        editable.move_left();
        editable.move_right();
        assert_eq!((12, 0), editable.text_range().cursor_position.unwrap());
    }
}
