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
}

#[cfg(test)]
mod test {
    use super::EditableText;

    #[test]
    fn advances_column() {
        let editable = EditableText::new();
        assert!(editable.text().is_empty());
    }

    #[test]
    fn insert_chars_correctly() {
        let mut editable = EditableText::new();
        editable.insert('h');
        editable.insert('e');
        editable.insert('l');
        editable.insert('l');
        editable.insert('o');

        assert_eq!(5, editable.chars.len());
    }

    #[test]
    fn return_correct_text() {
        let mut editable = EditableText::new();
        editable.insert('h');
        editable.insert('e');
        editable.insert('l');
        editable.insert('l');
        editable.insert('o');

        assert_eq!("hello".to_owned(), editable.text());
    }

    #[test]
    fn removes_currectly() {
        let mut editable = EditableText::new();
        editable.insert('h');
        editable.insert('e');
        editable.insert('l');
        editable.insert('l');
        editable.insert('o');
        editable.delete();

        assert_eq!("hell".to_owned(), editable.text());
    }
}
