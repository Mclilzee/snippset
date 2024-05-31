use crate::editable_text::EditableText;

pub struct Section {
    prefix: String,
    suffix: Option<EditableText>,
}

impl Section {
    pub fn parse_content(content: &str) -> Vec<Section> {
        let chars = content.chars().collect::<Vec<char>>();
        let mut column = 0;
        let mut row = 0;
        let mut sections = Vec::new();
        let mut prefix = String::new();
        for i in 0..chars.len() - 1 {
            column += 1;
            if chars[i] == '{' && chars[i + 1] == '}' {
                sections.push(Section::editable(prefix));
                prefix = String::new();
            } else if chars[i] == '\r' {
                row += 1;
                column = 0;
            }
        }

        if !prefix.is_empty() {
            sections.push(Section::tail(prefix));
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
    fn return_string_as_section_prefix() {
        let sections = Section::parse_content("Hello this is content with no value");
        assert!(sections.is_empty());
    }

    #[test]
    fn return_correct_section() {
        let sections = Section::parse_content("Hello this is content with no value {}");
        assert!(!sections.is_empty());
    }
}
