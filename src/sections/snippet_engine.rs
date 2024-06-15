use super::{section_manager::SectionManager, section_printer::SectionPrinter};
use crossterm::{
    event::{read, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal,
};
use std::io;

pub struct SnippetEngine {
    title: String,
    manager: SectionManager,
    printer: SectionPrinter,
}

impl SnippetEngine {
    pub fn new(title: &str, snippet: &str) -> Self {
        Self {
            title: title.to_owned(),
            manager: SectionManager::new(snippet),
            printer: SectionPrinter::new(),
        }
    }

    pub fn start(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        self.printer.print_header(&self.title)?;
        loop {
            let _ = self
                .printer
                .print_body(&self.manager.sections, self.manager.active_index)?;
            match read()? {
                Event::Key(event) => {
                    if event.kind != KeyEventKind::Press {
                        continue;
                    }

                    if event.modifiers == KeyModifiers::CONTROL && event.code == KeyCode::Char('c')
                    {
                        break;
                    };

                    if self.handle_input(event.code).is_err() {
                        break;
                    };
                }
                Event::Resize(_, _) => {
                    let _ = self.printer.print_header(&self.title);
                }
                _ => (),
            }
        }

        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn handle_input(&mut self, keycode: KeyCode) -> Result<(), ()> {
        let editor = match self.manager.active_editable() {
            Some(ed) => ed,
            None => return Err(()),
        };

        match keycode {
            KeyCode::Char(c) => editor.insert(c),
            KeyCode::Left => editor.move_left(),
            KeyCode::Right => editor.move_right(),
            KeyCode::Backspace => editor.delete(),
            KeyCode::Enter => {
                editor.reset_cursor();
                self.manager.active_index += 1;
            }
            KeyCode::Esc => {
                if self.manager.active_index > 0 {
                    self.manager.active_index -= 1;
                }
            }
            _ => (),
        }

        Ok(())
    }
}
