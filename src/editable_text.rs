use crossterm::event::KeyCode;

pub struct EditableText {
    index: usize,
    chars: Vec<char>,
}

impl EditableText {
    pub fn new() -> Self {
        EditableText {
            index: 0,
            chars: Vec::new(),
        }
    }

    pub fn insert(&mut self, c: char) {
        self.chars.push(c);
        self.index += 1;
    }

    pub fn remove(&mut self) {
        if !self.chars.is_empty() {
            self.chars.remove(self.index);
            if self.index > 0 {
                self.index -= 1;
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
}
