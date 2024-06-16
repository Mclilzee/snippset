use crossterm::QueueableCommand;
use crossterm::{cursor, execute, style::Print, terminal};
use std::io::{self, stdout, Stdout, Write};

use crate::sections::section::Section;

const TITLE_PADDING: u16 = 2;
const LINE_POSITION_UNDER_HEADER: u16 = 1;

pub struct SectionPrinter {
    stdout: Stdout,
    body_row: u16,
    width: u16,
}

impl SectionPrinter {
    pub fn new() -> Self {
        Self {
            stdout: stdout(),
            body_row: 0,
            width: 0,
        }
    }

    pub fn print_header(&mut self, title: &str) -> io::Result<()> {
        let (width, _) = terminal::size()?;

        self.stdout
            .queue(cursor::MoveTo(0, 0))?
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))?
            .queue(Print(format!("Snippet: {title}\n")))?
            .queue(cursor::MoveDown(2))?
            .queue(cursor::MoveToColumn(0))?
            .queue(Print((0..width).map(|_| '-').collect::<String>()))?
            .queue(cursor::MoveDown(1))?
            .queue(cursor::MoveToColumn(0))?
            .queue(cursor::SavePosition)?;

        self.stdout.flush()?;
        Ok(())
    }

    pub fn print_body(&mut self, sections: &[Section], cursor_index: usize) -> io::Result<()> {
        for section in sections.iter() {
            for (i, c) in section.prefix.iter().enumerate() {
                self.stdout.queue(Print(c))?;
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
