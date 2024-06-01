pub struct Editable {
    cursor: usize,
    prefix: String,
    suffix: Vec<char>,
}

impl Editable {
    pub fn new(prefix: String) -> Self {
        Editable {
            cursor: 0,
            prefix,
            suffix: Vec::new(),
        }
    }

    pub fn insert(&mut self, c: char) {
        self.suffix.push(c);
        self.cursor += 1;
    }

    pub fn delete(&mut self) {
        if !self.suffix.is_empty() {
            self.suffix.remove(self.cursor - 1);
            if self.cursor > 0 {
                self.cursor -= 1;
            }
        }
    }

    pub fn text(&self) -> String {
        self.suffix.iter().collect()
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
            if self.suffix[i] == '\r' {
                row += 1;
                column = 0;
            }
        }

        (column, row)
    }
}

#[cfg(test)]
mod test {
    use super::Editable;

    #[test]
    fn advances_column() {
        let editable = create_editable("");
        assert!(editable.text().is_empty());
    }

    #[test]
    fn insert_chars_correctly() {
        let editable = create_editable("hello");
        assert_eq!(5, editable.suffix.len());
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
        let mut editable = create_editable("Prefix", "This is test\rwith \rnew lin\res");
        editable.cursor_left();
        editable.cursor_left();
        editable.cursor_left();
        let (column, row) = editable.get_cursor_position();
        assert_eq!(column, 7);
        assert_eq!(row, 2);
    }

    fn create_editable(prefix: &str, suffix: &str) -> Editable {
        let mut editable = Editable::new(prefix.to_owned());
        editable.suffix = suffix.chars().collect();
        editable
    }
}
