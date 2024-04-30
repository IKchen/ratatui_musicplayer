use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph};
use crate::action::Action;
use crate::components::Component;
use crate::error::MyError;

#[derive(Clone)]
pub struct PlayZone{
    pub process: u32,
}
impl PlayZone{
    pub fn new()->Self{
        let process=0;
        Self{process}
    }
}
impl Component for PlayZone{
    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<(), MyError> {
        let paragraph=Paragraph::new("播放信息")
            .block( Block::new()
                .title("播放信息").red()
                .borders(Borders::ALL)).blue();
        f.render_widget(paragraph,rect);
        Ok(())
    }

    fn update(&mut self, action: Option<Action>) -> Result<(), MyError> {
        todo!()
    }

}