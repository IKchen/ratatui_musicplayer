use ratatui::{prelude::*, widgets::*};
use ratatui::prelude::Direction::Vertical;
use crate::error::MyError;
use super::Component;
use  crate::components ::tracinglog;
use crate::components::tracinglog::TracingLog;

pub struct Home {
    component_name:String,
}
impl  Home{
    pub fn new() -> Self {
        let component_name ="none".to_string();
        Self {component_name }
    }

}
impl Component for Home{
    fn draw(&mut self ,f:&mut ratatui::Frame<'_>,rect: Rect)->Result<(),MyError>{
        let layout=Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
            .split(f.size());
        f.render_widget(Paragraph::new("内容1")
                            .block( Block::new()
                                .title("标题1").red()
                                .borders(Borders::ALL)).blue().on_green(), layout[0]);

        let mut tracinglog=TracingLog::new();
        tracinglog.draw(f,layout[1]);
        Ok(())
    }
    fn update(& mut self)->Result<(),MyError>{
        Ok(())
    }
}