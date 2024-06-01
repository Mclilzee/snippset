#[derive(PartialEq)]
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
        self.cursor += 1;
        self.suffix.insert(self.cursor - 1, c);
    }

    pub fn delete(&mut self) {
        if self.cursor > 0 {
            self.suffix.remove(self.cursor - 1);
            self.cursor -= 1;
        }
    }

    pub fn text(&self) -> String {
        format!("{}{}", self.prefix, self.suffix.iter().collect::<String>())
    }

    pub fn cursor_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    pub fn cursor_right(&mut self) {
        if self.cursor < self.suffix.len() {
            self.cursor += 1;
        }
    }

    pub fn terminal_cursor_position(&self) -> (u16, u16) {
        let mut column = 0;
        let mut row = 0;
        self.prefix.chars().for_each(|c| {
            column += 1;
            if c == '\r' {
                row += 1;
                column = 0;
            }
        });

        self.suffix
            .iter()
            .enumerate()
            .take_while(|(i, _)| i < &self.cursor)
            .map(|(_, c)| c)
            .for_each(|c| {
                column += 1;
                if c == &'\r' {
                    row += 1;
                    column = 0;
                }
            });

        (column, row)
    }
}

#[cfg(test)]
mod test {
    use super::Editable;

    #[test]
    fn advances_column() {
        let editable = create_editable("", "");
        assert!(editable.text().is_empty());
    }

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
        let mut editable = create_editable("this is ", "my friend");
        editable.cursor_left();
        editable.cursor_left();
        editable.cursor_left();
        editable.delete();
        editable.delete();
        editable.cursor_left();
        editable.insert('s');
        assert_eq!("this is my sfend".to_owned(), editable.text());
    }

    #[test]
    fn moves_cursor_right() {
        let mut editable = create_editable("ignore me ", "new text to see");
        editable.cursor_left();
        editable.cursor_left();
        editable.cursor_left();
        editable.cursor_left();
        editable.cursor_right();
        editable.delete();
        editable.delete();
        editable.cursor_right();
        editable.insert('r');
        assert_eq!("ignore me new text tsree".to_owned(), editable.text());
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
        editable.cursor_left();
        editable.cursor_left();
        editable.cursor_left();
        let (column, row) = editable.terminal_cursor_position();
        assert_eq!(column, 11);
        assert_eq!(row, 0);
    }

    #[test]
    fn get_cursor_after_moving_forward() {
        let mut editable = create_editable("first ", "ano ther");
        editable.cursor_left();
        editable.cursor_left();
        editable.cursor_left();
        editable.cursor_right();
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
        editable.cursor_left();
        editable.cursor_left();
        editable.cursor_left();
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
