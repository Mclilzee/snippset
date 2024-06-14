use std::io::{self, stdout, Stdout};

use crossterm::{
    cursor,
    event::{read, Event, KeyCode, KeyEventKind, KeyModifiers},
    execute,
    style::Print,
    terminal,
};

use super::section::Section;

pub struct SectionManager {
    editable_index: usize,
    stdout: Stdout,
    title: String,
    sections: Vec<Section>,
}

impl SectionManager {
    pub fn new(title: &str, snippet: &str) -> Self {
        SectionManager {
            editable_index: 0,
            stdout: stdout(),
            sections: SectionManager::parse_content(snippet),
            title: title.to_owned(),
        }
    }

    fn parse_content(content: &str) -> Vec<Section> {
        let chars = content.chars().collect::<Vec<char>>();
        let mut sections = Vec::new();
        let mut prefix = Vec::new();
        for c in chars {
            if c != '}' {
                prefix.push(c);
                continue;
            }

            if let Some(c) = prefix.last() {
                if c == &'{' {
                    prefix.pop();
                    sections.push(Section::static_text(prefix));
                    sections.push(Section::editable());
                    prefix = Vec::new();
                } else {
                    prefix.push(*c);
                }
            }
        }
        sections.push(Section::static_text(prefix));
        sections
    }

    pub fn start(&mut self) -> io::Result<()> {
        terminal::enable_raw_mode()?;
        let mut stdout = stdout();
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

                    let section = self.get_next_active_editable();

                    match event.code {
                        KeyCode::Char(c) => section.insert(c),
                        KeyCode::Left => section.move_left(),
                        KeyCode::Right => section.move_right(),
                        KeyCode::Enter => {
                            section.reset_cursor();
                            section_index += 1;
                        }
                        KeyCode::Backspace => section.delete(),
                        KeyCode::Esc => {
                            section_index = if section_index > 0 {
                                section_index - 1
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

        execute!(
            stdout,
            cursor::MoveTo(0, 0),
            terminal::Clear(terminal::ClearType::FromCursorDown),
            Print(sections.iter().map(|s| s.text()).collect::<String>())
        )?;

        terminal::disable_raw_mode()?;
        Ok(())
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

    fn get_next_active_editable(&mut self) -> Option<&Section> {
        self.sections
            .iter()
            .filter(|s| s.is_editable())
            .skip(self.editable_index)
            .next()
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
        let expected = create_static("text");
        assert_eq!(section, &expected);
    }

    #[test]
    fn contains_tail_even_if_empty() {
        let manager = SectionManager::new("title", "");
        assert_eq!(1, manager.sections.len());
        let section = manager.sections.first().unwrap();
        let expected = create_static("");
        assert_eq!(section, &expected);
    }

    #[test]
    fn return_correct_section() {
        let manager = SectionManager::new("header", "Content {}");
        assert_eq!(3, manager.sections.len());
        let first = manager.sections.first().unwrap();
        let second = manager.sections.get(1).unwrap();
        let last = manager.sections.get(2).unwrap();

        let first_expected = create_static("Content ");
        let second_expected = Section::editable();
        let last_expected = create_static("");

        assert_eq!(first, &first_expected);
        assert_eq!(second, &second_expected);
        assert_eq!(last, &last_expected);
    }

    #[test]
    fn parse_multiple_sections_including_tail() {
        let manager = SectionManager::new("title", "Hello {}, another{} tail moving forward.");
        assert_eq!(5, manager.sections.len());

        let first = manager.sections.first().unwrap();
        let second = manager.sections.get(1).unwrap();
        let third = manager.sections.get(2).unwrap();
        let fourth = manager.sections.get(3).unwrap();
        let last = manager.sections.get(4).unwrap();
        assert_eq!(first, &create_static("Hello "));
        assert_eq!(second, &Section::editable());
        assert_eq!(third, &create_static(", another"));
        assert_eq!(fourth, &Section::editable());
        assert_eq!(last, &create_static(" tail moving forward."));
    }

    fn create_static(str: &str) -> Section {
        Section::static_text(str.chars().collect())
    }
}
