use super::editable_text::EditableText;

#[derive(Debug, PartialEq)]
pub struct Section {
    pub prefix: String,
    pub suffix: Option<EditableText>,
}

impl Section {
    pub fn body(prefix: String) -> Self {
        Section {
            prefix,
            suffix: Some(EditableText::new()),
        }
    }

    pub fn tail(prefix: String) -> Self {
        Section {
            prefix,
            suffix: None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Section;

    #[test]
    fn creates_correct_body() {
        let section = Section::body("Hello this".chars().collect());
        assert_eq!(section.prefix, "Hello this");
        assert!(section.suffix.is_some())
    }

    #[test]
    fn creates_tail() {
        let section = Section::tail("Hello world".chars().collect());
        assert_eq!(section.prefix, "Hello world");
        assert!(section.suffix.is_none());
    }
}
