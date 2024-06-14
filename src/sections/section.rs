use crate::text::{text::EditableText, text::StaticText};

#[derive(Debug, PartialEq)]
pub enum Section {
    Editable(EditableText),
    StaticText(StaticText),
}

impl Section {
    pub fn editable() -> Self {
        Self::Editable(EditableText::new())
    }

    pub fn static_text(chars: Vec<char>) -> Self {
        Self::StaticText(StaticText::new(chars))
    }

    fn chars(&self) -> &Vec<char> {
        match self {
            Self::Editable(ed) => &ed.chars,
            Self::StaticText(text) => &text.chars,
        }
    }
}

#[cfg(test)]
mod test {
    use super::Section;

    #[test]
    fn get_static_text() {
        let section = Section::static_text("Hello this".chars().collect());
        assert_eq!(
            section.chars(),
            &"Hello this".chars().collect::<Vec<char>>()
        );
    }

    #[test]
    fn get_editable_text() {
        let mut section = Section::editable();
        match section {
            Section::Editable(ref mut txt) => {
                "This is editable".chars().for_each(|c| txt.insert(c))
            }
            _ => {}
        }

        assert_eq!(
            section.chars(),
            &"This is editable".chars().collect::<Vec<char>>()
        );
    }
}
