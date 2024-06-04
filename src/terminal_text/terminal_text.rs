use super::{editable_text::EditableText, static_text::StaticText, TextRange};

pub enum TerminalText {
    Editable(EditableText),
    StaticText(StaticText),
}

impl TerminalText {
    pub fn editable() -> Self {
        Self::Editable(EditableText::new())
    }

    pub fn static_text(text: &str) -> Self {
        Self::StaticText(StaticText::new(text))
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
