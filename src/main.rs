mod section;

use section::Section;
use std::collections::HashMap;
use std::io::{self, stdout, Stdout, Write};

use crossterm::cursor;
use crossterm::event::{KeyEventKind, KeyModifiers};
use crossterm::style::Print;
use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use crossterm::{execute, terminal};
use inquire::{InquireError, Select};

fn main() -> Result<(), InquireError> {
    execute!(
        stdout(),
        terminal::Clear(terminal::ClearType::All),
        cursor::EnableBlinking,
        cursor::MoveTo(0, 0)
    )?;
    let mut map = HashMap::new();
    map.insert(
        "hello snippet",
        "This hello is a snippet {} placeholder {} lets go? {}",
    );

    let key = Select::new("Choose snippet", map.keys().collect()).prompt()?;
    let snippet = map.get(key).unwrap();

    enable_raw_mode()?;
    if let Err(e) = handle_snippet(key, snippet) {
        println!("Error: {:?}\r", e);
    }

    disable_raw_mode()?;
    Ok(())
}

fn handle_snippet(title: &str, snippet: &str) -> io::Result<()> {
    let mut stdout = stdout();
    print_initial_state(snippet, title, &mut stdout)?;
    let mut current_section = 0;

    loop {
        print_current_snippet(snippet, sections, &mut stdout)
        if let Event::Key(event) = read()? {
            if event.kind != KeyEventKind::Press {
                continue;
            }

            if event.code == KeyCode::Esc
                || (event.modifiers == KeyModifiers::CONTROL && event.code == KeyCode::Char('c'))
            {
                break;
            };
        }
    }

    execute!(stdout, cursor::DisableBlinking)?;
    Ok(())
}

fn print_initial_state(snippet: &str, title: &str, stdout: &mut Stdout) -> io::Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::FromCursorDown),
        Print(format!("Snippet: {title}\r")),
        cursor::MoveDown(1),
        Print("--------------------------------------\r"),
        cursor::MoveDown(1),
        Print(snippet),
    )?;

    Ok(())
}

fn print_current_snippet(
    snippet: &str,
    sections: &[Section],
    stdout: &mut Stdout,
) -> io::Result<()> {
    execute!(
        stdout,
        cursor::SavePosition,
        cursor::MoveTo(0, 3),
        terminal::Clear(terminal::ClearType::FromCursorDown),
        Print(snippet),
        cursor::RestorePosition
    )?;

    Ok(())
}
