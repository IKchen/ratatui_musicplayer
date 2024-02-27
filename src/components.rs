use std::sync::{Arc};
use ratatui::layout::Rect;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::Mutex;
use crate::action::Action;
use crate::app::App;
use crate::error::MyError;
use crate::tracing::TracingLog;
//引入组件
pub mod home;

pub mod quit;
pub mod tracinglog;
pub mod filelist;
pub mod playzone;
pub mod analysis;


pub trait Component{

    fn  draw(& mut self, f:&mut ratatui::Frame<'_>,rect:Rect)->Result<(),MyError>;
    fn  update(& mut self,action:Option<Action> )->Result<(),MyError>;
    fn init(&mut self) -> Result<(),MyError> {
        Ok(())
    }
    //注册 事件接收器
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) {

    }

}