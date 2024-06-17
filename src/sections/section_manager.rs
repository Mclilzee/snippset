use super::{editable_text::EditableText, section::Section};

pub struct SectionManager {
    pub sections: Vec<Section>,
    pub active_index: usize,
}

impl SectionManager {
    pub fn new(snippet: &str) -> Self {
        SectionManager {
            sections: SectionManager::parse_content(snippet),
            active_index: 0,
        }
    }

    fn parse_content(content: &str) -> Vec<Section> {
        let chars = content.chars().collect::<Vec<char>>();
        let mut sections = Vec::new();
        let mut static_txt = Vec::new();
        for c in chars {
            if c == '\r' {
                static_txt.push('\n');
            } else if c == '}' {
                if let Some(c) = static_txt.last() {
                    if c == &'{' {
                        static_txt.pop();
                        sections.push(Section::body(static_txt));
                        static_txt = Vec::new();
                    } else {
                        static_txt.push(*c);
                    }
                }
            } else {
                static_txt.push(c);
            }
        }

        sections.push(Section::tail(static_txt));
        sections
    }

    pub fn active_editable(&mut self) -> Option<&mut EditableText> {
        let section = match self.sections.get_mut(self.active_index) {
            Some(s) => s,
            None => return None,
        };

        section.suffix.as_mut()
    }
}

#[cfg(test)]
mod test {
    use crate::sections::{section::Section, section_manager::SectionManager};

    #[test]
    fn return_string_as_section_tail() {
        let manager = SectionManager::new("text");
        assert_eq!(1, manager.sections.len());
        let section = manager.sections.first().unwrap();
        let expected = section_tail("text");
        assert_eq!(section, &expected);
    }

    #[test]
    fn contains_tail_even_if_empty() {
        let manager = SectionManager::new("");
        assert_eq!(1, manager.sections.len());
        let section = manager.sections.first().unwrap();
        let expected = section_tail("");
        assert_eq!(section, &expected);
    }

    #[test]
    fn return_correct_section() {
        let manager = SectionManager::new("Content {}");
        assert_eq!(2, manager.sections.len());
        let body = manager.sections.first().unwrap();
        let tail = manager.sections.get(1).unwrap();

        assert_eq!(body, &section_body("Content "));
        assert_eq!(tail, &section_tail(""));
    }

    #[test]
    fn parse_multiple_sections() {
        let manager = SectionManager::new("Hello {}, another{} tail moving forward.");
        assert_eq!(3, manager.sections.len());

        let first = manager.sections.first().unwrap();
        let second = manager.sections.get(1).unwrap();
        let tail = manager.sections.get(2).unwrap();

        assert_eq!(first, &section_body("Hello "));
        assert_eq!(second, &section_body(", another"));
        assert_eq!(tail, &section_tail(" tail moving forward."));
    }

    #[test]
    fn replaces_windows_newline() {
        let manager = SectionManager::new("Content\r {} \r new line \r");
        assert_eq!(2, manager.sections.len());
        let body = manager.sections.first().unwrap();
        let tail = manager.sections.get(1).unwrap();

        assert_eq!(body, &section_body("Content\n "));
        assert_eq!(tail, &section_tail(" \n new line \n"));
    }

    fn section_body(str: &str) -> Section {
        Section::body(str.chars().collect())
    }

    fn section_tail(str: &str) -> Section {
        Section::tail(str.chars().collect())
    }
}
