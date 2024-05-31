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

        if !prefix.is_empty() {
            sections.push(Section::tail(prefix.iter().collect()));
        }

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
    fn return_correct_section() {
        let sections = Section::parse_content("Hello this is content with no value {}");
        assert!(!sections.is_empty());
    }

    #[test]
    fn parse_multiple_sections_including_tail() {
        let sections = Section::parse_content(
            "Hello this is content with no value {} This gonna be tail moving forward.",
        );
        assert!(sections.first().unwrap().suffix.is_some());
        assert!(sections.get(1).unwrap().suffix.is_none());
    }
}
