use std::io::{self, stdout, Stdout};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    style::Print,
    terminal,
};

use super::{editable_text::EditableText, section::Section};

pub struct SectionManager {
    stdout: Stdout,
    title: String,
    sections: Vec<Section>,
    active_index: usize,
}

impl SectionManager {
    pub fn new(title: &str, snippet: &str) -> Self {
        SectionManager {
            sections: SectionManager::parse_content(snippet),
            active_index: 0,
            stdout: stdout(),
            title: title.to_owned(),
        }
    }

    fn parse_content(content: &str) -> Vec<Section> {
        let chars = content.chars().collect::<Vec<char>>();
        let mut sections = Vec::new();
        let mut static_txt = Vec::new();
        for c in chars {
            if c != '}' {
                static_txt.push(c);
                continue;
            }

            if let Some(c) = static_txt.last() {
                if c == &'{' {
                    static_txt.pop();
                    sections.push(Section::body(static_txt));
                    static_txt = Vec::new();
                } else {
                    static_txt.push(*c);
                }
            }
        }

        sections.push(Section::tail(static_txt));
        sections
    }

    pub fn start(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        self.print_title()?;

        loop {
            self.print_snippet()?;
            match read()? {
                Event::Key(event) => {
                    if event.kind != KeyEventKind::Press {
                        continue;
                    }

                    if event.modifiers == KeyModifiers::CONTROL && event.code == KeyCode::Char('c')
                    {
                        break;
                    };

                    let ed = match self.active_editable() {
                        Some(ed) => ed,
                        None => break,
                    };

                    match event.code {
                        KeyCode::Char(c) => ed.insert(c),
                        KeyCode::Left => ed.move_left(),
                        KeyCode::Right => ed.move_right(),
                        KeyCode::Backspace => ed.delete(),
                        KeyCode::Enter => {
                            ed.reset_cursor();
                            self.active_index += 1;
                        }
                        KeyCode::Esc => {
                            self.active_index = if self.active_index > 0 {
                                self.active_index - 1
                            } else {
                                0
                            };
                        }
                        _ => (),
                    }
                }

                Event::Resize(_, _) => {
                    self.print_title()?;
                }
                _ => (),
            }
        }

        terminal::disable_raw_mode()?;
        Ok(())
    }

    fn active_editable(&mut self) -> Option<&mut EditableText> {
        let section = match self.sections.get_mut(self.active_index) {
            Some(s) => s,
            None => return None,
        };

        section.suffix.as_mut()
    }

    fn print_title(&self) -> io::Result<()> {
        execute!(
            &self.stdout,
            cursor::MoveTo(0, 0),
            terminal::Clear(terminal::ClearType::FromCursorDown),
            Print(format!("Snippet: {}\r", self.title)),
            cursor::MoveDown(1),
            Print("--------------------------------------\r"),
        )?;

        Ok(())
    }

    fn print_snippet(&self) -> io::Result<()> {
        execute!(
            &self.stdout,
            cursor::MoveTo(0, 5),
            terminal::Clear(terminal::ClearType::FromCursorDown),
            Print(self.sections.iter().map(|s| s.text()).collect::<String>())
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::sections::{section::Section, section_manager::SectionManager};

    #[test]
    fn return_string_as_section_tail() {
        let manager = SectionManager::new("title", "text");
        assert_eq!(1, manager.sections.len());
        let section = manager.sections.first().unwrap();
        let expected = section_tail("text");
        assert_eq!(section, &expected);
    }

    #[test]
    fn contains_tail_even_if_empty() {
        let manager = SectionManager::new("title", "");
        assert_eq!(1, manager.sections.len());
        let section = manager.sections.first().unwrap();
        let expected = section_tail("");
        assert_eq!(section, &expected);
    }

    #[test]
    fn return_correct_section() {
        let manager = SectionManager::new("header", "Content {}");
        assert_eq!(2, manager.sections.len());
        let body = manager.sections.first().unwrap();
        let tail = manager.sections.get(1).unwrap();

        assert_eq!(body, &section_body("Content "));
        assert_eq!(tail, &section_tail(""));
    }

    #[test]
    fn parse_multiple_sections() {
        let manager = SectionManager::new("title", "Hello {}, another{} tail moving forward.");
        assert_eq!(3, manager.sections.len());

        let first = manager.sections.first().unwrap();
        let second = manager.sections.get(1).unwrap();
        let tail = manager.sections.get(2).unwrap();

        assert_eq!(first, &section_body("Hello "));
        assert_eq!(second, &section_body(", another"));
        assert_eq!(tail, &section_tail(" tail moving forward."));
    }

    fn section_body(str: &str) -> Section {
        Section::body(str.chars().collect())
    }

    fn section_tail(str: &str) -> Section {
        Section::tail(str.chars().collect())
    }
}
