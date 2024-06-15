use std::io::{self, stdout};

use crossterm::{cursor, execute, style::Print, terminal};

use crate::sections::section::Section;

const TITLE_PADDING: u16 = 5;

pub fn print_sections(title: &str, sections: &[Section]) -> io::Result<()> {
    print_title(title)?;
    print_snippet(sections)?;

    Ok(())
}

fn print_snippet(sections: &[Section]) -> io::Result<()> {
    execute!(
        stdout(),
        cursor::MoveTo(0, TITLE_PADDING),
        terminal::Clear(terminal::ClearType::FromCursorDown),
        Print(sections.iter().map(|s| s.text()).collect::<String>())
    )?;

    Ok(())
}

fn print_title(title: &str) -> io::Result<()> {
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
