use crossterm::QueueableCommand;
use crossterm::{cursor, execute, style::Print, terminal};
use std::io::{self, stdout, Stdout, Write};

use crate::sections::section::Section;

const TITLE_PADDING: u16 = 2;

pub struct SectionPrinter {
    stdout: Stdout,
}

impl SectionPrinter {
    pub fn new() -> Self {
        Self { stdout: stdout() }
    }

    pub fn print_header(&mut self, title: &str) -> io::Result<()> {
        let (width, _) = terminal::size()?;
        execute!(
            self.stdout,
            cursor::MoveTo(0, 0),
            terminal::Clear(terminal::ClearType::FromCursorDown),
            Print(format!("Snippet: {title}")),
            cursor::MoveDown(1),
            cursor::MoveToColumn(0),
            Print((0..width).map(|_| '-').collect::<String>()),
            cursor::MoveDown(TITLE_PADDING),
            cursor::MoveToColumn(0),
            cursor::SavePosition
        )?;
        Ok(())
    }

    pub fn print_body(&mut self, sections: &[Section], cursor_index: usize) -> io::Result<()> {
        for section in sections.iter() {
            for (i, c) in section.prefix.iter().enumerate() {
                self.stdout
                    .queue(cursor::RestorePosition)?
                    .queue(Print(c))?;
            }

            let suffix = match section.suffix.as_ref() {
                Some(ed) => ed,
                None => continue,
            };

            for c in &suffix.chars {}
        }
        execute!(
            self.stdout,
            terminal::Clear(terminal::ClearType::FromCursorDown),
            Print(sections.iter().map(|s| s.text()).collect::<String>())
        )?;

        Ok(())
    }
}
