use crate::editable_text::EditableText;

pub struct Section {
    prefix: String,
    suffix: Option<EditableText>,
}

impl Section {
    pub fn parse_content(content: &str) -> Vec<Section> {
        let chars = content.chars().collect::<Vec<char>>();
        let mut sections = Vec::new();
        let mut prefix = Vec::new();
        for c in chars {
            if c != '}' {
                prefix.push(c);
                continue;
            }

            if let Some(c) = prefix.last() {
                if c == &'{' {
                    prefix.pop();
                    sections.push(Section::editable(prefix.iter().collect()));
                    prefix = Vec::new();
                } else {
                    prefix.push(*c);
                }
            }
        }
        sections.push(Section::tail(prefix.iter().collect()));
        sections
    }

    fn tail(prefix: String) -> Self {
        Section {
            prefix,
            suffix: None,
        }
    }

    fn editable(prefix: String) -> Self {
        Section {
            prefix,
            suffix: Some(EditableText::new()),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Section;

    #[test]
    fn return_string_as_section_tail() {
        let sections = Section::parse_content("Hello this is content with no value");
        assert!(sections.first().unwrap().suffix.is_none());
    }

    #[test]
    fn always_contains_tail() {
        let sections = Section::parse_content("");
        assert!(sections.first().unwrap().suffix.is_none());
    }

    #[test]
    fn return_correct_section() {
        let sections = Section::parse_content("Hello this is content with no value {}");
        assert!(sections.len() == 2);
        assert!(sections.first().unwrap().suffix.is_some());
        assert!(sections.get(1).unwrap().suffix.is_none());
    }

    #[test]
    fn parse_multiple_sections_including_tail() {
        let sections = Section::parse_content("Hello content {}, another{} tail moving forward.");
        let first_section = sections.first().unwrap();
        let second_section = sections.get(1).unwrap();
        let tail = sections.get(2).unwrap();
        assert_eq!(first_section.prefix, "Hello content ".to_owned());
        assert_eq!(second_section.prefix, ", another".to_owned());
        assert_eq!(tail.prefix, " tail moving forward.".to_owned());
    }
}
