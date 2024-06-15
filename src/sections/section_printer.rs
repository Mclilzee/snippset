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

    pub fn print_body(&mut self, sections: &[Section], cursor_index: usize) -> io::Result<()> {
        execute!(
            self.stdout,
            cursor::MoveTo(0 as u16, self.body_row),
            terminal::Clear(terminal::ClearType::FromCursorDown),
            Print(sections.iter().map(|s| s.text()).collect::<String>())
        )?;

        Ok(())
    }

    pub fn print_header(&mut self, title: &str) -> io::Result<()> {
        self.width = terminal::size()?.0;
        self.stdout
            .queue(cursor::MoveTo(0, 0))?
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))?;

        let mut column = 0;
        let mut row = 0;

        for c in "Snippet: ".chars().chain(title.chars()).chain("\r".chars()) {
            if c == '\r' {
                row += 1;
                column = 0;
            } else {
                column += 1;
            }

            if column > self.width {
                row += 1;
                column = 0;
            }
            self.stdout.queue(Print(c))?;
        }

        let _ = self
            .stdout
            .queue(cursor::MoveDown(row))?
            .queue(Print((0..self.width).map(|_| '-').collect::<String>()))?;

        self.body_row = row + TITLE_PADDING + LINE_POSITION_UNDER_HEADER;
        self.stdout.flush()?;
        Ok(())
    }
}
