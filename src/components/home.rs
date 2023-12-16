use ratatui::{prelude::*, widgets::*};
use ratatui::prelude::Direction::Vertical;
use crate::error::MyError;
use super::Component;
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
        let layout=Layout::default()
            .direction(Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(Rect::new(0, 0, 10, 10));
        f.render_widget(Paragraph::new("内容1")
                            .block( Block::new()
                                .title("标题1").red()
                                .borders(Borders::ALL)).blue().on_green(), layout[0]);
        f.render_widget(Paragraph::new("内容2")
                            .block( Block::new()
                                .title("标题2").red()
                                .borders(Borders::ALL)), layout[1]);
        Ok(())
    }
    fn update(& mut self)->Result<(),MyError>{
        Ok(())
    }
}