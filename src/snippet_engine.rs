use crate::sections::section_manager::SectionManager;
use crossterm::event::{read, Event, KeyCode, KeyEventKind, KeyModifiers};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget},
    Frame,
};
use std::io;

pub struct SnippetEngine {
    title: String,
    manager: SectionManager,
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
            .padding(ratatui::widgets::Padding::uniform(1))
            .title_bottom(instructions.centered())
            .border_set(border::DOUBLE);

        let text: Vec<_> = self
            .manager
            .sections
            .iter()
            .enumerate()
            .flat_map(|(i, s)| {
                let mut suffix: Vec<Span> = match s.suffix.as_ref() {
                    Some(editable) => std::iter::once('[')
                        .chain(editable.chars())
                        .chain(std::iter::once(']'))
                        .map(|s| s.to_string().bold().yellow().underlined())
                        .collect(),
                    None => vec![],
                };

                if self.manager.active_index == i && s.suffix.is_some() {
                    match s
                        .suffix
                        .as_ref()
                        // +1 since we add extra characters to our string
                        .and_then(|e| suffix.get_mut(e.insertion_position() + 1))
                    {
                        Some(s) => s.style.bg = Some(Color::White),
                        None => suffix.last_mut().unwrap().style.bg = Some(Color::White),
                    }
                }

                Line::from_iter(
                    s.prefix
                        .iter()
                        .map(|c| Span::from(c.to_string()))
                        .chain(suffix),
                )
            })
            .collect();

        Paragraph::new(Line::from(text))
            .left_aligned()
            .wrap(ratatui::widgets::Wrap { trim: true })
            .block(block)
            .render(area, buf);
    }
}

impl SnippetEngine {
    pub fn new(title: &str, snippet: &str) -> Self {
        Self {
            title: title.to_owned(),
            manager: SectionManager::new(snippet),
        }
    }

    pub fn start(&mut self) -> io::Result<String> {
        let mut terminal = ratatui::init();
        loop {
            terminal.draw(|frame| self.draw(frame))?;
            if let Event::Key(event) = read()? {
                if event.kind != KeyEventKind::Press {
                    continue;
                }

                if event.modifiers == KeyModifiers::CONTROL && event.code == KeyCode::Char('c') {
                    break;
                };

                if self.handle_input(event.code).is_err() {
                    break;
                };
            }
        }

        ratatui::restore();
        Ok(self.manager.text())
    }

    fn handle_input(&mut self, keycode: KeyCode) -> Result<(), String> {
        let editor = match self.manager.active_editable() {
            Some(ed) => ed,
            None => return Err("Couldn't retrieve editable section".into()),
        };

        match keycode {
            KeyCode::Char(c) => editor.insert(c),
            KeyCode::Left => editor.move_left(),
            KeyCode::Right => editor.move_right(),
            KeyCode::Backspace => editor.delete(),
            KeyCode::Esc => self.manager.previous_section().unwrap_or_default(),
            KeyCode::Enter => self.manager.next_section()?,
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
    use insta::assert_snapshot;
    use ratatui::{backend::TestBackend, Terminal};

    #[test]
    fn test_render_app() {
        let app = SnippetEngine::new("This is a title", "This is the body text");
        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal
            .draw(|frame| frame.render_widget(&app, frame.area()))
            .unwrap();
        assert_snapshot!(terminal.backend());
    }
}
