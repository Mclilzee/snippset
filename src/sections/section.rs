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

    pub fn chars(&self) -> Vec<char> {
        todo!()
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
        " World".chars().for_each(|c| editable.insert(c));
        section.suffix = Some(editable);
        assert_eq!("Hello World".to_owned(), section.text());
    }
}
