use ratatui::{
    buffer::Buffer,
    layout::Rect,
    prelude::CrosstermBackend,
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
    Frame, Terminal,
};
use std::io::Stdout;

use super::section_manager::SectionManager;

pub struct TerminalRender<'a> {
    terminal: Terminal<CrosstermBackend<Stdout>>,
    title: &'a str,
    manager: &'a SectionManager
}

impl Widget for &TerminalRender<'_> {
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
            .padding(ratatui::widgets::Padding::top(1))
            .title_bottom(instructions.centered())
            .border_set(border::THICK);

        let text = Text::from(self.manager.display_text());

        Paragraph::new(text)
            .centered()
            .block(block)
            .render(area, buf);
    }
}

impl TerminalRender<'_> {

    fn new(title: &str, manager: &SectionManager) -> Self {
        Self {
            terminal: ratatui::init(),
            title,
            manager,
        }
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
        let app = TerminalRender::new("This is a title", "This is the body text");
        let mut terminal = Terminal::new(TestBackend::new(80, 20)).unwrap();
        terminal
            .draw(|frame| frame.render_widget(&app, frame.area()))
            .unwrap();
        assert_snapshot!(terminal.backend());
    }
}
