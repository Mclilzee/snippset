pub struct EditableText {
    cursor: usize,
    chars: Vec<char>,
}

impl EditableText {
    pub fn new() -> Self {
        EditableText {
            cursor: 0,
            chars: Vec::new(),
        }
    }

    pub fn insert(&mut self, c: char) {
        self.chars.push(c);
        self.cursor += 1;
    }

    pub fn delete(&mut self) {
        if !self.chars.is_empty() {
            self.chars.remove(self.cursor - 1);
            if self.cursor > 0 {
                self.cursor -= 1;
            }
        }
    }

    pub fn text(&self) -> String {
        self.chars.iter().collect()
    }

    pub fn cursor_left(&mut self) {
        self.cursor -= 1;
    }

    pub fn cursor_right(&mut self) {
        self.cursor += 1;
    }

    pub fn get_cursor_position(&self) -> (u16, u16) {
        let mut column = 0;
        let mut row = 0;
        for i in 0..self.cursor {
            column += 1;
            if self.chars[i] == '\r' {
                row += 1;
                column = 0;
            }
        }

        (column, row)
    }
}

#[cfg(test)]
mod test {
    use super::EditableText;

    #[test]
    fn advances_column() {
        let editable = create_editable("");
        assert!(editable.text().is_empty());
    }

    #[test]
    fn insert_chars_correctly() {
        let editable = create_editable("hello");
        assert_eq!(5, editable.chars.len());
    }

    #[test]
    fn return_correct_text() {
        let editable = create_editable("hello");
        assert_eq!("hello".to_owned(), editable.text());
    }

    #[test]
    fn handles_newlines() {
        let editable = create_editable("hel\rlo");
        assert_eq!("hel\rlo".to_owned(), editable.text());
    }

    #[test]
    fn removes_correctly() {
        let mut editable = create_editable("hello");
        editable.delete();
        editable.delete();
        editable.delete();
        assert_eq!("he".to_owned(), editable.text());
    }

    #[test]
    fn moves_cursor_left() {
        let mut editable = create_editable("my friend");
        editable.cursor_left();
        editable.cursor_left();
        editable.cursor_left();
        editable.delete();
        editable.delete();
        assert_eq!("my fend".to_owned(), editable.text());
    }

    #[test]
    fn moves_cursor_right() {
        let mut editable = create_editable("new text to see");
        editable.cursor_left();
        editable.cursor_left();
        editable.cursor_left();
        editable.cursor_left();
        editable.cursor_right();
        editable.delete();
        editable.delete();
        assert_eq!("new text tsee".to_owned(), editable.text());
    }

    #[test]
    fn get_cursor_position() {
        let editable = create_editable("This is test with new lines");
        let (column, row) = editable.get_cursor_position();
        assert_eq!(column, 27);
        assert_eq!(row, 0);
    }

    #[test]
    fn get_cursor_after_moving() {
        let mut editable = create_editable("This is test with new lines");
        editable.cursor_left();
        editable.cursor_left();
        editable.cursor_left();
        let (column, row) = editable.get_cursor_position();
        assert_eq!(column, 24);
        assert_eq!(row, 0);
    }

    #[test]
    fn get_cursor_with_newline() {
        let editable = create_editable("This is test\rwith \rnew lin\res");
        let (column, row) = editable.get_cursor_position();
        assert_eq!(column, 2);
        assert_eq!(row, 3);
    }

    #[test]
    fn get_cursor_with_newline_moving_backward() {
        let mut editable = create_editable("This is test\rwith \rnew lin\res");
        editable.cursor_left();
        editable.cursor_left();
        editable.cursor_left();
        let (column, row) = editable.get_cursor_position();
        assert_eq!(column, 7);
        assert_eq!(row, 2);
    }

    fn create_editable(text: &str) -> EditableText {
        let mut editable = EditableText::new();
        text.chars().for_each(|c| editable.insert(c));
        editable
    }
}
