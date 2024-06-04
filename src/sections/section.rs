use crate::text::{editable_text::EditableText, static_text::StaticText, TextRange};

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

    pub fn range(&self, width: u16) -> TextRange {
        let mut row = 0;
        let mut column = 0;
        let mut text = String::with_capacity(self.chars().len());
        let mut cursor_position = None;
        for (i, c) in self.chars().iter().enumerate() {
            text.push(*c);
            if c == &'\r' || column > width {
                row += 1;
                column = 0;
            } else {
                column += 1;
            }

            if let Self::Editable(ed) = self {
                if ed.cursor == i {
                    cursor_position = Some((column, row));
                }
            }
        }

        TextRange::new((column, row), text, cursor_position)
    }

    fn chars(&self) -> &Vec<char> {
        match self {
            Self::Editable(ed) => &ed.chars,
            Self::StaticText(text) => &text.chars,
        }
    }
}
//
// #[cfg(test)]
// mod test {
//     use super::Section;
//
//     #[test]
//     fn get_full_text_tail() {
//         let section = Section::static_text("Hello this".to_string());
//         assert_eq!(section.text(), "Hello this".to_owned());
//     }
//
//     #[test]
//     fn get_full_text_editable() {
//         let mut section = Section::editable();
//         fill_editable(&mut section, "stuff goes here");
//         assert_eq!(section.text(), "stuff goes here".to_owned());
//     }
//
//     fn fill_editable(section: &mut Section, string: &str) {
//         if let Section::Editable(ed) = section {
//             string.chars().for_each(|c| ed.insert(c));
//         }
//     }
// }
