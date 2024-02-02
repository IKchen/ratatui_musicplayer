use std::sync::{Arc};
use ratatui::{prelude::*, widgets::*};
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::Mutex;
use tracing::info;
use crate::action::Action;
use crate::app::App;
use crate::error::MyError;
use super::Component;
use  crate::components::tracinglog::TracingLogComponent;
use crate::tracing::TracingLog;

pub struct Home {
    component_name:String,
    pub log:String,
    pub action_tx: Option<UnboundedSender<Action>>,
}
impl  Home{
    pub fn new(log:String) -> Self {
        let component_name ="none".to_string();
        let log=log;
        let action_tx =None;
        Self {component_name,log ,action_tx}
    }

}
impl Component for Home{
     fn draw(&mut self, f:&mut ratatui::Frame<'_>, rect: Rect) ->Result<(),MyError>{
        // let layout=Layout::new(
        //     Direction::Vertical,
        //     [Constraint::Percentage(70), Constraint::Percentage(30)],
        // )
        //     .split(f.size());

        let mut sub_layout=Layout::new(
            Direction::Horizontal,
            [Constraint::Percentage(25),Constraint::Percentage(75)],
        ).split(rect);

        f.render_widget(Paragraph::new("文件")
                            .block( Block::new()
                                .title("文件列表").red()
                                .borders(Borders::ALL)).blue(), sub_layout[0]);

        f.render_widget(Paragraph::new("播放区")
                            .block( Block::new()
                                .title("播放区").red()
                                .borders(Borders::ALL)).blue(), sub_layout[1]);

        //获取tracinglog struct实例
        // let mut log=self.log.clone();
        // let mut tracinglog=TracingLogComponent::new(log);
        //  tracinglog.register_action_handler();
        // tracinglog.draw(f,layout[1],app)?;
        Ok(())
    }
    fn update(& mut self,action:Option<Action>)->Result<(),MyError>{

        Ok(())
    }
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>){
        self.action_tx = Some(tx);

    }
}