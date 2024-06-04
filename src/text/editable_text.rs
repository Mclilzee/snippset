use super::TextRange;

#[derive(Debug, PartialEq)]
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
        self.cursor += 1;
        self.chars.insert(self.cursor - 1, c);
    }

    pub fn delete(&mut self) {
        if self.cursor > 0 {
            self.chars.remove(self.cursor - 1);
            self.cursor -= 1;
        }
    }

    pub fn move_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.cursor < self.chars.len() {
            self.cursor += 1;
        }
    }

    pub fn reset_cursor(&mut self) {
        self.cursor = self.chars.len();
    }

    pub fn text_range(&self) -> TextRange {
        let str = self.chars.iter().collect();
        let end_position = self.chars.len() as u16;

        TextRange::new(
            (end_position, 0),
            str,
            Some((end_position - self.cursor as u16, 0)),
        )
    }
}

#[cfg(test)]
mod test {
    use super::EditableText;

    #[test]
    fn initalize_correctly() {
        let editable = create_editable("prefix", "hello");
        assert_eq!("hello".chars().collect::<Vec<char>>(), editable.suffix);
        assert_eq!("prefix".to_owned(), editable.prefix);
    }

    #[test]
    fn return_correct_text() {
        let editable = create_editable("hello ", "world");
        assert_eq!("hello world".to_owned(), editable.text());
    }

    #[test]
    fn text_contains_newlines() {
        let editable = create_editable("he\ry ", "frie\rnd");
        assert_eq!("he\ry frie\rnd".to_owned(), editable.text());
    }

    #[test]
    fn return_text_placeholder_if_empty() {
        let editable = create_editable("hello ", "");
        assert_eq!("hello _".to_owned(), editable.text());
    }

    #[test]
    fn insert_new_characters() {
        let mut editable = create_editable("just prefix ", "");
        editable.insert('w');
        editable.insert('o');
        editable.insert('w');
        assert_eq!("just prefix wow".to_owned(), editable.text());
    }

    #[test]
    fn removes_correctly() {
        let mut editable = create_editable("first one", " second");
        editable.delete();
        editable.delete();
        editable.delete();
        assert_eq!("first one sec".to_owned(), editable.text());
    }

    #[test]
    fn moves_cursor_left() {
        let mut editable = create_editable("text", "my test");
        assert_eq!(7, editable.cursor);
        editable.move_left();
        editable.move_left();
        editable.move_left();
        assert_eq!(4, editable.cursor);
    }

    #[test]
    fn moves_cursor_left_respects_boundary() {
        let mut editable = create_editable("text", "c");
        assert_eq!(1, editable.cursor);
        editable.move_left();
        editable.move_left();
        editable.move_left();
        editable.move_left();
        assert_eq!(0, editable.cursor);
    }

    #[test]
    fn moves_cursor_right_respects_boundary() {
        let mut editable = create_editable("text", "c");
        assert_eq!(1, editable.cursor);
        editable.move_left();
        editable.move_right();
        editable.move_right();
        editable.move_right();
        assert_eq!(1, editable.cursor);
    }

    #[test]
    fn moves_cursor_right() {
        let mut editable = create_editable("new text", "O ok");
        assert_eq!(4, editable.cursor);
        editable.move_left();
        editable.move_left();
        editable.move_right();
        assert_eq!(3, editable.cursor);
    }

    #[test]
    fn resets_cursor_position() {
        let mut editable = create_editable("ok", "cursor");
        assert_eq!(6, editable.cursor);
        editable.move_left();
        editable.move_left();
        assert_eq!(4, editable.cursor);
        editable.reset_cursor();
        assert_eq!(6, editable.cursor);
    }

    #[test]
    fn deletes_at_cursor_position() {
        let mut editable = create_editable("this is ", "my friend");
        editable.move_left();
        editable.move_left();
        editable.move_left();
        editable.delete();
        editable.delete();
        assert_eq!("this is my fend".to_owned(), editable.text());
    }

    #[test]
    fn insert_at_cursor_position() {
        let mut editable = create_editable("another", " one");
        editable.move_left();
        editable.move_left();
        editable.insert('s');
        assert_eq!("another osne".to_owned(), editable.text());
    }

    #[test]
    fn get_cursor_position() {
        let editable = create_editable("first ", "ano ther");
        let (column, row) = editable.terminal_cursor_position();
        assert_eq!(column, 14);
        assert_eq!(row, 0);
    }

    #[test]
    fn get_cursor_after_moving() {
        let mut editable = create_editable("first ", "ano ther");
        editable.move_left();
        editable.move_left();
        editable.move_left();
        let (column, row) = editable.terminal_cursor_position();
        assert_eq!(column, 11);
        assert_eq!(row, 0);
    }

    #[test]
    fn get_cursor_after_moving_forward() {
        let mut editable = create_editable("first ", "ano ther");
        editable.move_left();
        editable.move_left();
        editable.move_left();
        editable.move_right();
        let (column, row) = editable.terminal_cursor_position();
        assert_eq!(column, 12);
        assert_eq!(row, 0);
    }

    #[test]
    fn get_cursor_with_newline() {
        let editable = create_editable("new\rline", " test\r\rnew lin\res");
        let (column, row) = editable.terminal_cursor_position();
        assert_eq!(column, 2);
        assert_eq!(row, 4);
    }

    #[test]
    fn get_cursor_with_newline_moving_backward() {
        let mut editable = create_editable("Prefix", "This is test\rwith \rnew lin\res");
        editable.move_left();
        editable.move_left();
        editable.move_left();
        let (column, row) = editable.terminal_cursor_position();
        assert_eq!(column, 7);
        assert_eq!(row, 2);
    }

    fn create_editable(prefix: &str, suffix: &str) -> Editable {
        let mut editable = Editable::new(prefix.to_owned());
        suffix.chars().for_each(|c| editable.insert(c));
        editable
    }
}
