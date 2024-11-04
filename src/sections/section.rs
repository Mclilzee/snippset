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
        let prefix = self.prefix.iter().collect::<String>();
        let suffix = self.suffix.as_ref().map(|e| e.text()).unwrap_or_default();

        format!("{prefix}{suffix}")
    }

    pub fn len(&self) -> usize {
        self.prefix.len() + self.suffix.as_ref().map(|e| e.len()).unwrap_or_default()
    }
}

#[cfg(test)]
mod test {
    use crate::sections::editable_text::EditableText;

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

    #[test]
    fn final_text() {
        let mut section = Section::body("Hello".chars().collect());
        let mut editable = EditableText::new();
        editable.chars = " World".chars().collect::<Vec<char>>();
        section.suffix = Some(editable);
        assert_eq!("Hello World".to_owned(), section.text());
    }

    #[test]
    fn length() {
        let mut section = Section::body("Some text goes".chars().collect());
        let mut editable = EditableText::new();
        editable.chars = "with".chars().collect::<Vec<char>>();
        section.suffix = Some(editable);
        assert_eq!(section.len(), 18);
    }

    #[test]
    fn empty_editable() {
        let mut section = Section::body("Some ".chars().collect());
        section.suffix = None;
        assert_eq!(section.len(), 5);
    }

    #[test]
    fn empty_length() {
        let mut section = Section::body("".chars().collect());
        section.suffix = None;
        assert_eq!(section.len(), 0);
    }
}
