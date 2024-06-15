use crossterm::QueueableCommand;
use crossterm::{cursor, execute, style::Print, terminal};
use std::io::{self, stdout, Stdout};

use crate::sections::section::Section;

const TITLE_PADDING: u16 = 2;

pub struct SectionPrinter {
    stdout: Stdout,
    snippet_start: (u16, u16),
}

impl SectionPrinter {
    pub fn new() -> Self {
        Self {
            stdout: stdout(),
            snippet_start: (0, 0),
        }
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

    pub fn print_header(&mut self, title: &str) -> io::Result<()> {
        let (width, _) = terminal::size()?;
        self.stdout
            .queue(cursor::MoveTo(0, 0))?
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))?
            .queue(Print("Snippet: "))?;

        let mut column = 0;
        let mut row = 0;

        for c in title.chars() {
            if c == '\n' {
                row += 1;
                column = 0;
            } else {
                column += 1;
            }

            if column > width {
                row += 1;
                column = 0;
            }
            self.stdout.queue(Print(c));
        }

        self.stdout
            .queue(Print("\r"))?
            .queue(cursor::MoveDown(1))?
            .queue(Print("--------------------------------------\r"))?;

        self.snippet_start = (column, row + TITLE_PADDING);
        Ok(())
    }
}
