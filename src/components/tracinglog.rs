use std::ptr::addr_of_mut;
use std::sync::{Arc};
use ratatui::{prelude::*, widgets::*};
use tokio::runtime::Runtime;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::Mutex;
use crate::error::MyError;
use super::Component;

use tracing::{Subscriber, Event, event, Level, info, warn};
use crate::action::Action;
use crate::app::App;

use crate::tracing::TracingLog;

pub struct TracingLogComponent{
    pub logs: String,
    pub vertical_scroll_state: ScrollbarState,
    pub horizontal_scroll_state: ScrollbarState,
    pub vertical_scroll: usize,
    pub horizontal_scroll: usize,
    pub action_tx: Option<UnboundedSender<Action>>,
}

impl TracingLogComponent {
    pub fn new(log:String)->Self{
        let logs =log;
        let vertical_scroll =1;
        let vertical_scroll_state=ratatui::widgets::ScrollbarState::new(20);
        let horizontal_scroll_state=ratatui::widgets::ScrollbarState::new(20);
        let horizontal_scroll=1;
        let action_tx =None;
        Self{logs,vertical_scroll,vertical_scroll_state,horizontal_scroll,horizontal_scroll_state,action_tx}
    }
    pub fn set_log(&mut self ,log:String){
        self.logs=log;
    }
}
impl Component for TracingLogComponent{
     fn draw(&mut self, f:&mut ratatui::Frame<'_>, rect: Rect) ->Result<(),MyError>{
        // 这里我们使用克隆的Arc来访问日志
      //  let app_clone =Arc::clone(&app);
        let text = self.logs.clone();
       // println!("text is {:?}",text);
         self.vertical_scroll_state = self.vertical_scroll_state.content_length(text.len());
        let layout=Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(100)],
        )
            .split(rect);
        info!("绘制图形");
         // let rt = Runtime::new().unwrap();
         // let mut text="".to_string();
         // // 在同步函数中执行异步函数
         // rt.block_on(async {
         //     // 你的异步代码
         //    text=(*text_log.lock().await).clone();
         //
         // });
        f.render_widget(Paragraph::new(text)
                            .block( Block::new()
                                .title("tracing日志").red()
                                .borders(Borders::ALL))
                            .blue()
                            .scroll((self.vertical_scroll as u16, 0)),

                        layout[0]);
         f.render_stateful_widget(
             Scrollbar::default()
                 .orientation(ScrollbarOrientation::VerticalRight)
                 .begin_symbol(Some("↑"))
                 .end_symbol(Some("↓")),
             layout[0],
             &mut self.vertical_scroll_state,
         );

         Ok(())
    }
    fn update(& mut self, action: Option<Action>) ->Result<(),MyError>{
        match action {
            Some(Action::Quit)=> return Ok(()),
            Some(Action::Down) => {
                self.vertical_scroll = self.vertical_scroll.saturating_add(1);
                self.vertical_scroll_state =
                    self.vertical_scroll_state.position(self.vertical_scroll);
            }
            Some(Action::Up) => {
                self.vertical_scroll = self.vertical_scroll.saturating_sub(1);
                self.vertical_scroll_state =
                    self.vertical_scroll_state.position(self.vertical_scroll);
            }
            Some(Action::Left) => {
                self.horizontal_scroll = self.horizontal_scroll.saturating_sub(1);
                self.horizontal_scroll_state =
                    self.horizontal_scroll_state.position(self.horizontal_scroll);
            }
            Some(Action::Right) => {
                self.horizontal_scroll = self.horizontal_scroll.saturating_add(1);
                self.horizontal_scroll_state =
                    self.horizontal_scroll_state.position(self.horizontal_scroll);
            }
            _ => {}
        }
        self.action_tx.clone().unwrap().send(Action::Render)?;
        Ok(())
    }
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>){
        self.action_tx = Some(tx);

    }

}