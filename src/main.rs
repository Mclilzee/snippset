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
        let event = read()?;
        match event {
            Event::Key(event) if event.kind == KeyEventKind::Press => {
                print!("Key pressed: ");
                if event.modifiers != KeyModifiers::NONE {
                    print!("{:?}+", event.modifiers);
                }
                println!("{:?}\r", event.code);
                if event.code == KeyCode::Esc {
                    break;
                }
            }
            _ => {}
        }
    }
    Ok(())
}
