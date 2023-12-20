use futures::future::ok;
use ratatui::{prelude::*, widgets::*};
use ratatui::prelude::Direction::Vertical;
use crate::error::MyError;
use super::Component;

pub struct Quit {
    component_name:String,
}

impl Quit{
    pub fn new() -> Self {
        let component_name ="none".to_string();
        Self {component_name }
    }
}

impl Component for Quit{
    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<(), MyError> {
        f.render_widget(Clear, f.size());
        let exit_block=Block::default()
            .title("Y/N")
            .borders(Borders::NONE)
            .style(Style::default().bg(Color::DarkGray));
        let exit_text=Text::styled("you want to quit ?(y/n)",
                                   Style::default().fg(Color::Red),);
        let exit_paragraph = Paragraph::new(exit_text)
            .block(exit_block)
            .wrap(Wrap { trim: false });
        let area = centered_rect(60, 25, f.size());
        f.render_widget(exit_paragraph,area);
        Ok(())
    }
    fn update(&mut self) -> Result<(), MyError> {
        Ok(())
    }
}



//中间的弹框
/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    // Cut the given rectangle into three vertical pieces
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    // Then cut the middle vertical piece into three width-wise pieces
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1] // Return the middle chunk
}