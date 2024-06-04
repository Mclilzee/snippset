mod sections;
mod text;

use std::collections::HashMap;
use std::io::{self, stdout, Stdout};

use crossterm::cursor;
use crossterm::event::{KeyEventKind, KeyModifiers};
use crossterm::style::Print;
use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use crossterm::{execute, terminal};
use inquire::{InquireError, Select};
use sections::section::Section;
use sections::section_manager::SectionManager;

const TITLE_HEIGHT: u16 = 5;

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
        "This hello is a snippet {} placeholder {} lets go? {} nice",
    );
    map.insert("Another snippet", "BEHAVE YOURSELF {} Please.");

    let key = Select::new("Choose snippet", map.keys().collect()).prompt()?;
    let snippet = map.get(key).unwrap();
    let mut section_manager = SectionManager::new(key, snippet);

    enable_raw_mode()?;
    if let Err(e) = section_manager.start() {
        println!("Error: {:?}\r", e);
    }

    disable_raw_mode()?;
    Ok(())
}

fn print_title(title: &str, stdout: &mut Stdout) -> io::Result<()> {
    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::FromCursorDown),
        Print(format!("Snippet: {title}\r")),
        cursor::MoveDown(1),
        Print("--------------------------------------\r"),
    )?;

    Ok(())
}
