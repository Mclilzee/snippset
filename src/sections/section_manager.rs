use std::io::{self, stdout, Stdout};

use super::section::Section;

pub struct SectionManager {
    editable_section: usize,
    stdout: Stdout,
    title: String,
    sections: Vec<Section>,
}

impl SectionManager {
    pub fn new(title: &str, snippet: &str) -> Self {
        SectionManager {
            editable_section: 0,
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
        let mut stdout = stdout();
        // print_title(title, &mut stdout)?;
        // let mut section_index = 0;
        // let mut sections = SectionManager::parse_content(snippet);
        //
        // loop {
        //     print_snippet(&sections, section_index, &mut stdout)?;
        //     match read()? {
        //         Event::Key(event) => {
        //             if event.kind != KeyEventKind::Press {
        //                 continue;
        //             }
        //
        //             if event.modifiers == KeyModifiers::CONTROL && event.code == KeyCode::Char('c')
        //             {
        //                 break;
        //             };
        //
        //             let section = match sections.get_mut(section_index).unwrap() {
        //                 Section::Tail(_) => break,
        //                 Section::Body(editable) => editable,
        //             };
        //
        //             match event.code {
        //                 KeyCode::Char(c) => section.insert(c),
        //                 KeyCode::Left => section.move_left(),
        //                 KeyCode::Right => section.move_right(),
        //                 KeyCode::Enter => {
        //                     section.reset_cursor();
        //                     section_index += 1;
        //                 }
        //                 KeyCode::Backspace => section.delete(),
        //                 KeyCode::Esc => {
        //                     section_index = if section_index > 0 {
        //                         section_index - 1
        //                     } else {
        //                         0
        //                     };
        //                 }
        //                 _ => (),
        //             }
        //         }
        //         Event::Resize(_, _) => {
        //             print_title(title, &mut stdout)?;
        //         }
        //         _ => (),
        //     }
        // }

        //     execute!(
        //         stdout,
        //         cursor::MoveTo(0, 0),
        //         terminal::Clear(terminal::ClearType::FromCursorDown),
        //         Print(sections.iter().map(|s| s.text()).collect::<String>())
        //     )?;
        //
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use crate::sections::{section::Section, section_manager::SectionManager};

    #[test]
    fn return_string_as_section_tail() {
        let manager = SectionManager::new("title", "text");
        let section = manager.sections.first().unwrap();
        let expected = create_static("text");
        assert_eq!(section, &expected);
    }

    #[test]
    fn always_contains_tail() {
        let manager = SectionManager::new("title", "");
        let section = manager.sections.first().unwrap();
        let expected = create_static("");
        assert_eq!(section, &expected);
    }

    #[test]
    fn return_correct_section() {
        let manager = SectionManager::new("header", "Content {}");
        assert_eq!(2, manager.sections.len());
        let first = manager.sections.first().unwrap();
        let second = manager.sections.get(2).unwrap();
        let last = manager.sections.get(3).unwrap();

        let first_expected = create_static("Content ");
        let second_expected = create_editable("");
        let last_expected = create_static("");

        assert_eq!(first, &first_expected);
        assert_eq!(second, &second_expected);
        assert_eq!(last, &last_expected);
    }

    #[test]
    fn parse_multiple_sections_including_tail() {
        let manager = SectionManager::new("title", "Hello {}, another{} tail moving forward.");
        let first = manager.sections.first().unwrap();
        let second = manager.sections.get(1).unwrap();
        let tail = manager.sections.get(2).unwrap();
        assert_eq!(first, &Section::body("Hello content ".to_owned()));
        assert_eq!(second, &Section::body(", another".to_owned()));
        assert_eq!(tail, &Section::tail(" tail moving forward.".to_owned()));
    }

    fn create_static(str: &str) -> Section {
        Section::static_text(str.chars().collect())
    }

    fn create_editable(str: &str) -> Section {
        let mut section = Section::editable();
        if let Section::Editable(ref mut ed) = section {
            str.chars().for_each(|c| ed.insert(c));
        }

        section
    }
}
