use crate::editable_text::Editable;

#[derive(PartialEq)]
pub enum Section {
    Body(Editable),
    Tail(String),
}

impl Section {
    pub fn parse_content(content: &str) -> Vec<Self> {
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
        Section::Tail(prefix)
    }

    fn editable(prefix: String) -> Self {
        Section::Body(Editable::new(prefix))
    }

    pub fn text(&self) -> String {
        match self {
            Section::Body(editable) => editable.text(),
            Section::Tail(str) => str.to_owned(),
        }
    }
}

#[cfg(test)]
mod test {
    use super::Section;

    #[test]
    fn return_string_as_section_tail() {
        let sections = Section::parse_content("text");
        assert!(sections.first().unwrap() == &Section::Tail("text".to_owned()));
    }
    //
    // #[test]
    // fn always_contains_tail() {
    //     let sections = Section::parse_content("");
    //     assert!(sections.first().unwrap().suffix.is_none());
    // }
    //
    // #[test]
    // fn return_correct_section() {
    //     let sections = Section::parse_content("Hello this is content with no value {}");
    //     assert!(sections.len() == 2);
    //     assert!(sections.first().unwrap().suffix.is_some());
    //     assert!(sections.get(1).unwrap().suffix.is_none());
    // }
    //
    // #[test]
    // fn parse_multiple_sections_including_tail() {
    //     let sections = Section::parse_content("Hello content {}, another{} tail moving forward.");
    //     let first_section = sections.first().unwrap();
    //     let second_section = sections.get(1).unwrap();
    //     let tail = sections.get(2).unwrap();
    //     assert_eq!(first_section.prefix, "Hello content ".to_owned());
    //     assert_eq!(second_section.prefix, ", another".to_owned());
    //     assert_eq!(tail.prefix, " tail moving forward.".to_owned());
    // }
    //
    // #[test]
    // fn get_full_text_tail() {
    //     let section = Section::tail("Hello this".to_string());
    //     assert_eq!(section.text(), "Hello this".to_owned());
    // }
    //
    // #[test]
    // fn get_full_text_editable() {
    //     let mut section = Section::editable("Hello this ".to_string());
    //     fill_editable_suffix(&mut section, "suffix here");
    //     assert_eq!(section.text(), "Hello this suffix here".to_owned());
    // }
    //
    // fn fill_editable_suffix(section: &mut Section, string: &str) {
    //     let suffix = section.suffix.as_mut().unwrap();
    //     string.chars().for_each(|c| suffix.insert(c));
    // }
}
