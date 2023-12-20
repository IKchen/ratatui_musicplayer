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
            [Constraint::Percentage(80), Constraint::Percentage(20)],
        )
            .split(f.size());

        let mut sub_layout=Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(25),Constraint::Percentage(75)],
        ).split(layout[0]);

        f.render_widget(Paragraph::new("文件")
                            .block( Block::new()
                                .title("文件列表").red()
                                .borders(Borders::ALL)).blue(), sub_layout[0]);

        f.render_widget(Paragraph::new("播放区")
                            .block( Block::new()
                                .title("播放区").red()
                                .borders(Borders::ALL)).blue(), sub_layout[1]);

        let mut tracinglog=TracingLog::new();
        tracinglog.draw(f,layout[1]);
        Ok(())
    }
    fn update(& mut self)->Result<(),MyError>{
        Ok(())
    }
}