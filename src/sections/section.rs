use super::editable_text::EditableText;

#[derive(Debug, PartialEq)]
pub struct Section {
    prefix: Vec<char>,
    suffix: Option<EditableText>,
}

impl Section {
    pub fn body(prefix: &str) -> Self {
        Section {
            prefix: prefix.chars().collect(),
            suffix: Some(EditableText::new()),
        }
    }

    pub fn tail(prefix: &str) -> Self {
        Section {
            prefix: prefix.chars().collect(),
            suffix: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Section;

    #[test]
    fn creates_correct_body() {
        let section = Section::body("Hello this");
        assert_eq!(section.prefix, "Hello this".chars().collect::<Vec<char>>());
        assert!(section.suffix.is_some())
    }

    #[test]
    fn creates_tail() {
        let section = Section::tail("Hello world");
        assert_eq!(section.prefix, "Hello world".chars().collect::<Vec<char>>());
        assert!(section.suffix.is_none());
    }
}
