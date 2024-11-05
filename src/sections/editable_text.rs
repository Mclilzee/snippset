use ratatui::{
    crossterm::style::Stylize,
    text::{Line, Span},
};

#[derive(Debug, PartialEq, Eq)]
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

    pub fn cursor_to_right_edge(&mut self) {
        self.cursor = self.chars.len();
    }

    pub fn text(&self) -> String {
        self.chars.iter().collect::<String>()
    }

    pub fn chars(&self) -> Vec<char> {
        self.chars.clone()
    }

    pub fn cursor(&self) -> Option<usize> {
        if self.chars.is_empty() {
            None
        } else {
            Some(self.cursor)
        }
    }
}

#[cfg(test)]
mod test {
    use super::EditableText;

    #[test]
    fn initalize_correctly() {
        let editable = create_editable("hello");
        assert_eq!("hello".chars().collect::<Vec<char>>(), editable.chars);
    }

    #[test]
    fn inserts_new_characters() {
        let mut editable = create_editable("");
        editable.insert('w');
        editable.insert('o');
        editable.insert('w');
        let result = editable.chars.iter().collect::<String>();
        assert_eq!("wow".to_owned(), result);
    }

    #[test]
    fn removes_correctly() {
        let mut editable = create_editable(" second");
        editable.delete();
        editable.delete();
        editable.delete();
        let result = editable.chars.iter().collect::<String>();
        assert_eq!(" sec".to_owned(), result);
    }

    #[test]
    fn moves_cursor_left() {
        let mut editable = create_editable("my test");
        assert_eq!(7, editable.cursor);
        editable.move_left();
        editable.move_left();
        editable.move_left();
        assert_eq!(4, editable.cursor);
    }

    #[test]
    fn moves_cursor_left_respects_boundary() {
        let mut editable = create_editable("c");
        assert_eq!(1, editable.cursor);
        editable.move_left();
        editable.move_left();
        editable.move_left();
        editable.move_left();
        assert_eq!(0, editable.cursor);
    }

    #[test]
    fn moves_cursor_right_respects_boundary() {
        let mut editable = create_editable("c");
        assert_eq!(1, editable.cursor);
        editable.move_left();
        editable.move_right();
        editable.move_right();
        editable.move_right();
        assert_eq!(1, editable.cursor);
    }

    #[test]
    fn moves_cursor_right() {
        let mut editable = create_editable("O ok");
        assert_eq!(4, editable.cursor);
        editable.move_left();
        editable.move_left();
        editable.move_right();
        assert_eq!(3, editable.cursor);
    }

    #[test]
    fn resets_cursor_position() {
        let mut editable = create_editable("cursor");
        assert_eq!(6, editable.cursor);
        editable.move_left();
        editable.move_left();
        assert_eq!(4, editable.cursor);
        editable.cursor_to_right_edge();
        assert_eq!(6, editable.cursor);
    }

    #[test]
    fn deletes_at_cursor_position() {
        let mut editable = create_editable("my friend");
        editable.move_left();
        editable.move_left();
        editable.move_left();
        editable.delete();
        editable.delete();
        let result = editable.chars.iter().collect::<String>();
        assert_eq!("my fend".to_owned(), result);
    }

    #[test]
    fn text() {
        let editable = create_editable("Hello this is the edit");
        assert_eq!("Hello this is the edit".to_owned(), editable.text())
    }
    #[test]
    fn insert_at_cursor_position() {
        let mut editable = create_editable("one");
        editable.move_left();
        editable.move_left();
        editable.insert('s');
        let result = editable.chars.iter().collect::<String>();
        assert_eq!("osne".to_owned(), result);
    }

    #[test]
    fn cursor_position() {
        let mut editable = create_editable("Some Text");
        editable.move_left();
        editable.move_left();
        editable.insert('s');
        let result = editable.chars.iter().collect::<String>();
        assert_eq!("osne".to_owned(), result);
    }

    fn create_editable(suffix: &str) -> EditableText {
        let mut editable = EditableText::new();
        suffix.chars().for_each(|c| editable.insert(c));
        editable
    }

}
