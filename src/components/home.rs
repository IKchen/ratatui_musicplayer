use std::sync::{Arc};
use ratatui::{prelude::*, widgets::*};
use ratatui::prelude::Direction::Vertical;
use tokio::sync::Mutex;
use tracing::info;
use crate::app::App;
use crate::error::MyError;
use super::Component;
use  crate::components::tracinglog::TracingLogComponent;
use crate::tracing::TracingLog;

pub struct Home {
    component_name:String,
   pub log:String
}
impl  Home{
    pub fn new(log:String) -> Self {
        let component_name ="none".to_string();
        let log=log;
        Self {component_name,log }
    }

}
impl Component for Home{
     fn draw(&mut self, f:&mut ratatui::Frame<'_>, rect: Rect, app:Arc<App>) ->Result<(),MyError>{
        let layout=Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(70), Constraint::Percentage(30)],
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

        //获取tracinglog struct实例
        let mut log=self.log.clone();
        let mut tracinglog=TracingLogComponent::new(log);
        tracinglog.draw(f,layout[1],app)?;
        Ok(())
    }
    fn update(& mut self)->Result<(),MyError>{
        Ok(())
    }
}