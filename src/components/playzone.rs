use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::Stylize;
use ratatui::widgets::{Block, Borders, Paragraph};
use tracing_subscriber::field::display::Messages;
use crate::action::Action;
use crate::components::Component;
use crate::error::MyError;

#[derive(Clone)]
pub struct PlayZone{
    pub playing_message: String,
}
impl PlayZone{
    pub fn new()->Self{
        let playing_message="暂无文件播放".to_string();
        Self{playing_message}
    }
    pub fn set_playing_message(&mut self,messages: String){
            self.playing_message=messages
    }
}
impl Component for PlayZone{
    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<(), MyError> {
        let paragraph=Paragraph::new(self.playing_message.clone())
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