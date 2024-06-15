use super::editable_text::EditableText;

#[derive(Debug, PartialEq)]
pub struct Section {
    pub prefix: Vec<char>,
    pub suffix: Option<EditableText>,
}

impl Section {
    pub fn body(prefix: Vec<char>) -> Self {
        Section {
            prefix,
            suffix: Some(EditableText::new()),
        }
    }

    pub fn tail(prefix: Vec<char>) -> Self {
        Section {
            prefix,
            suffix: None,
        }
    }

    pub fn text(&self) -> String {
        let mut result = self.prefix.clone();
        if let Some(ref suffix) = self.suffix {
            result.extend(&suffix.chars);
        }

        result.iter().collect()
    }
}

#[cfg(test)]
mod test {
    use super::Section;

    #[test]
    fn creates_correct_body() {
        let section = Section::body("Hello this".chars().collect());
        assert_eq!(section.prefix, "Hello this".chars().collect::<Vec<char>>());
        assert!(section.suffix.is_some())
    }

    #[test]
    fn creates_tail() {
        let section = Section::tail("Hello world".chars().collect());
        assert_eq!(section.prefix, "Hello world".chars().collect::<Vec<char>>());
        assert!(section.suffix.is_none());
    }
}
