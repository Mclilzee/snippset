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
    fn removes_currectly() {
        let mut editable = create_editable("hello");
        editable.delete();

        assert_eq!("hell".to_owned(), editable.text());
    }

    #[test]
    fn moves_cursor_left() {
        let mut editable = create_editable("hello");
        editable.cursor_left();
        editable.delete();

        assert_eq!("helo".to_owned(), editable.text());
    }

    fn moves_cursor_right() {}

    fn create_editable(text: &str) -> EditableText {
        let mut editable = EditableText::new();
        text.chars().for_each(|c| editable.insert(c));
        editable
    }
}
