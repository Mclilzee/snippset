use std::io::{self, stdout, Stdout};

use crossterm::{cursor, execute, style::Print, terminal};

use crate::sections::section::Section;

const TITLE_PADDING: u16 = 5;

pub struct SectionPrinter {
    stdout: Stdout,
}

impl SectionPrinter {
    pub fn new() -> Self {
        Self { stdout: stdout() }
    }

    pub fn print_snippet(&self, sections: &[Section]) -> io::Result<()> {
        execute!(
            stdout(),
            cursor::MoveTo(0, TITLE_PADDING),
            terminal::Clear(terminal::ClearType::FromCursorDown),
            Print(sections.iter().map(|s| s.text()).collect::<String>())
        )?;

        Ok(())
    }

    pub fn print_title(&mut self, title: &str) -> io::Result<()> {
        execute!(
            stdout(),
            cursor::MoveTo(0, 0),
            terminal::Clear(terminal::ClearType::FromCursorDown),
            Print(format!("Snippet: {}\r", title)),
            cursor::MoveDown(1),
            Print("--------------------------------------\r"),
        )?;

        Ok(())
    }
}
