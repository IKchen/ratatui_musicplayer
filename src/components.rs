
//components,rs 与文件夹components 同名，方便引用同名文件夹下的home.rs
use crossterm::event::{KeyEvent, MouseEvent};
use ratatui::layout::Rect;
use tokio::sync::mpsc::UnboundedSender;
use crate::error::MyError;
//引入组件
pub mod home;
pub mod tracinglog;

pub trait Component{

    fn  draw(& mut self, f:&mut ratatui::Frame<'_>,rect:Rect)->Result<(),MyError>;
    fn  update(& mut self)->Result<(),MyError>;

}