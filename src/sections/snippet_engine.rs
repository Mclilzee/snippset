use super::section_manager::SectionManager;
use crossterm::{event::{read, Event, KeyCode, KeyEventKind, KeyModifiers}, terminal};
use std::io;

pub struct SnippetEngine {
    title: String,
    manager: SectionManager,
}

impl SnippetEngine {
    pub fn new(title: &str, snippet: &str) -> Self {
        Self {
            title: title.to_owned(),
            manager: SectionManager::new(snippet)
        }

    }

    pub fn start(&mut self) -> io::Result<()> {
    terminal::enable_raw_mode()?;
        loop {
            match read()? {
                Event::Key(event) => {
                    if event.kind != KeyEventKind::Press {
                        continue;
                    }

                    if event.modifiers == KeyModifiers::CONTROL && event.code == KeyCode::Char('c')
                    {
                        break;
                    };

                    let ed = match self.manager.active_editable() {
                        Some(ed) => ed,
                        None => break,
                    };

                    KeyCode::Char(c) => ed.insert(c),
                    KeyCode::Left => ed.move_left(),
                    KeyCode::Right => ed.move_right(),
                    KeyCode::Backspace => ed.delete(),
                    KeyCode::Enter => {
                        ed.reset_cursor();
                        self.active_index += 1;
                    }
                    KeyCode::Esc => {
                        if self.active_index > 0 {
                            self.active_index -= 1;
                        }
                    }
                }

            Event::Resize(_, _) => {},

            }
        }

    terminal::disable_raw_mode()?;
        Ok(())
    }
}
