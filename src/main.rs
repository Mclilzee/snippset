mod editable;
mod section;
mod section_manager;

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
use section::Section;

const TITLE_HEIGHT: u16 = 4;

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
    map.insert("Another snippet", "BEHAVE YOURSELF {} Please.");

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
    print_title(title, &mut stdout)?;
    let mut section_index = 0;
    let mut sections = Section::parse_content(snippet);

    loop {
        print_snippet(&sections, section_index, &mut stdout)?;
        if let Event::Key(event) = read()? {
            if event.kind != KeyEventKind::Press {
                continue;
            }

            if event.code == KeyCode::Esc
                || (event.modifiers == KeyModifiers::CONTROL && event.code == KeyCode::Char('c'))
            {
                break;
            };

            let section = match sections.get_mut(section_index).unwrap() {
                Section::Tail(_) => break,
                Section::Body(editable) => editable,
            };

            match event.code {
                KeyCode::Char(c) => section.insert(c),
                KeyCode::Left => section.move_left(),
                KeyCode::Right => section.move_right(),
                KeyCode::Enter => {
                    section_index += 1;
                    if let Section::Tail(_) = sections.get(section_index).unwrap() {
                        break;
                    }
                }
                KeyCode::Backspace => section.delete(),
                KeyCode::Up => {
                    section_index = if section_index > 0 {
                        section_index - 1
                    } else {
                        0
                    };
                }
                _ => (),
            }
        }
    }

    execute!(
        stdout,
        cursor::MoveTo(0, 0),
        terminal::Clear(terminal::ClearType::FromCursorDown,),
        Print("Copied to clipboard.")
    )?;

    Ok(())
}

fn print_snippet(sections: &[Section], sec_index: usize, stdout: &mut Stdout) -> io::Result<()> {
    let mut text = String::new();
    let mut column = 0;
    let mut row = TITLE_HEIGHT;

    for (index, section) in sections.iter().enumerate() {
        text += &section.text();
        if let Section::Body(ed) = section {
            let (ed_col, ed_row) = ed.terminal_cursor_position();
            if index > sec_index {
                continue;
            }

            if ed_row > 0 {
                row += ed_row;
                column = ed_col;
            } else {
                column += ed_col;
            }
        }
    }

    row += column / terminal::size()?.0;
    column %= terminal::size()?.0;

    execute!(
        stdout,
        cursor::MoveTo(0, TITLE_HEIGHT),
        terminal::Clear(terminal::ClearType::FromCursorDown),
        Print(text),
        cursor::MoveTo(column, row)
    )?;

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
