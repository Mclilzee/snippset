use crate::sections::editable::Editable;

#[derive(Debug, PartialEq)]
pub enum Section {
    Body(Editable),
    Tail(String),
}

impl Section {
    pub fn tail(prefix: String) -> Self {
        Section::Tail(prefix)
    }

    pub fn editable(prefix: String) -> Self {
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
    fn get_full_text_tail() {
        let section = Section::tail("Hello this".to_string());
        assert_eq!(section.text(), "Hello this".to_owned());
    }

    #[test]
    fn get_full_text_editable() {
        let mut section = Section::editable("Hello this ".to_string());
        fill_editable_suffix(&mut section, "suffix here");
        assert_eq!(section.text(), "Hello this suffix here".to_owned());
    }

    fn fill_editable_suffix(section: &mut Section, string: &str) {
        if let Section::Body(editable) = section {
            string.chars().for_each(|c| editable.insert(c));
        }
    }
}
