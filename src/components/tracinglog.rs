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
    pub liststate:ListState,
}

impl TracingLogComponent {
    pub fn new(log:String)->Self{
        let logs =log;
        let vertical_scroll =2;
        let vertical_scroll_state=ratatui::widgets::ScrollbarState::new(20);
        let horizontal_scroll_state=ratatui::widgets::ScrollbarState::new(20);
        let horizontal_scroll=1;
        let action_tx =None;
        let mut liststate=ListState::default();
        Self{logs,vertical_scroll,vertical_scroll_state,horizontal_scroll,horizontal_scroll_state,action_tx,liststate}
    }
    pub fn set_log(&mut self ,log:String){
        self.logs=log;
    }
}
impl Component for TracingLogComponent{
     fn draw(&mut self, f:&mut ratatui::Frame<'_>, rect: Rect) ->Result<(),MyError>{
        // 这里我们使用克隆的Arc来访问日志
      //  let app_clone =Arc::clone(&app);
         let layout=Layout::new( Direction::Vertical,
                                 [Constraint::Percentage(100)],).split(rect);
        let text = self.logs.clone();
         let lines: Vec<String> = text.lines().map(|line| line.to_string()).collect();//将string 转化为vec
         let mut list_item:Vec<ListItem>=lines.iter().rev().map(//rev（）是倒序
             | log|{ListItem::new(
                 Line::from(vec![log.into()])
             ).green()}
         ).collect();

         let block=Block::new()
             .borders(Borders::NONE).padding(Padding::zero());
         let outblock=Block::new()
             .title("Tracing日志").red()
             .borders(Borders::ALL).padding(Padding::zero());

         let list = List::new(list_item)
             .block(block)
             .highlight_style(
                 Style::default()
                     .add_modifier(Modifier::BOLD)
                     .add_modifier(Modifier::REVERSED)
                     .fg(Color::Blue),
             )
             .highlight_symbol(">")
             .highlight_spacing(HighlightSpacing::Always);
        f.render_widget(outblock,rect);
         f.render_widget(list, layout[0].inner(&Margin::new(1,1)));
         Ok(())
    }
    fn update(& mut self, action: Option<Action>) ->Result<(),MyError>{
        match action {
            Some(Action::Quit)=> return Ok(()),
            Some(Action::Down) => {
                // self.vertical_scroll = self.vertical_scroll.saturating_add(1);
                // self.liststate=self.liststate.clone().with_offset(self.vertical_scroll);
                //println!("self vc is {:?}",self.liststate.offset());
                //self.vertical_scroll = self.vertical_scroll.saturating_add(1);
                // self.vertical_scroll_state =
                //     self.vertical_scroll_state.position(self.vertical_scroll);
                //self.vertical_scroll_state.next();
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