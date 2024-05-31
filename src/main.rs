use std::io;

use crossterm::event::{KeyEventKind, KeyModifiers};
use crossterm::{
    event::{read, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use inquire::{InquireError, Select};

fn main() -> Result<(), InquireError> {
    Select::new("Choose snippet", snippets_list()).prompt()?;

    enable_raw_mode()?;
    if let Err(e) = print_events() {
        println!("Error: {:?}\r", e);
    }

    disable_raw_mode()?;
    Ok(())
}

fn snippets_list() -> Vec<String> {
    vec!["First snippet".into(), "Other snippet".into()]
}

fn print_events() -> io::Result<()> {
    loop {
        if let Event::Key(event) = read()? {
            if event.kind == KeyEventKind::Press && event.code == KeyCode::Esc {
                break;
            }

            if event.kind == KeyEventKind::Press {
                println!("{event:?}");
            }
        }
    }
    Ok(())
}
