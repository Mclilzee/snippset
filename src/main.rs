mod sections;
mod text;

use std::collections::HashMap;
use std::io::stdout;

use crossterm::cursor;
use crossterm::{execute, terminal};
use inquire::{InquireError, Select};
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

    if let Err(e) = section_manager.start() {
        println!("Error: {:?}\r", e);
    }
    Ok(())
}
