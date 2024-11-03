use super::{section_manager::SectionManager, section_printer::SectionPrinter};
use crossterm::{
    event::{read, Event, KeyCode, KeyEventKind, KeyModifiers},
    terminal,
};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    DefaultTerminal, Frame,
};
use std::io;

pub struct SnippetEngine {
    title: String,
    manager: SectionManager,
    printer: SectionPrinter,
}

impl Widget for &SnippetEngine {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(format!(" {} ", self.title).bold());
        let instructions = Line::from(vec![
            " Next Snipp ".into(),
            "<Enter>".blue().bold(),
            " Previous Snipp ".into(),
            "<Esc> ".blue().bold(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let text = Text::from(vec![Line::from(vec!["Some text".into()])]);

        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl SnippetEngine {
    pub fn new(title: &str, snippet: &str) -> Self {
        Self {
            title: title.to_owned(),
            manager: SectionManager::new(snippet),
            printer: SectionPrinter::new(),
        }
    }

    pub fn start(&mut self) -> io::Result<String> {
        terminal::enable_raw_mode()?;
        self.printer.print_header(&self.title)?;
        loop {
            self.printer
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
        Ok(self.manager.text())
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

    fn draw(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::{backend::TestBackend, Terminal};
    use insta::assert_snapshot;

    #[test]
    fn test_render_app() {
        let app = SnippetEngine::new("This is a title", "This is the body text");
        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal.draw(|frame| frame.render_widget(&app, frame.area())).unwrap();
        assert_snapshot!(terminal.backend());
    }
}
