use crossterm::style::{Attribute, SetAttribute};
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
        self.stdout
            .queue(cursor::RestorePosition)?
            .queue(terminal::Clear(terminal::ClearType::FromCursorDown))?;

        let empty_cell_chars = vec!['{', '}'];

        let mut position = (0, 0);
        for (i, section) in sections.iter().enumerate() {
            self.stdout
                .queue(SetAttribute(Attribute::Reset))?
                .queue(Print(&section.prefix))?;
            let ed = match section.suffix.as_ref() {
                Some(ed) => ed,
                None => continue,
            };

            let chars = if ed.chars.is_empty() {
                &empty_cell_chars
            } else {
                &ed.chars
            };

            self.stdout.queue(SetAttribute(Attribute::Underlined))?;
            if i == cursor_index {
                position = cursor::position()?;
                for (i, c) in chars.iter().enumerate() {
                    self.stdout.queue(Print(c))?;

                    if i < ed.cursor {
                        position = cursor::position()?;
                    }
                }
            } else {
                self.stdout.queue(Print(chars.iter().collect::<String>()))?;
            }
        }

        println!("{:?}", position);
        self.stdout.queue(cursor::MoveTo(position.0, position.1))?;
        self.stdout.flush()?;
        Ok(())
    }
}
