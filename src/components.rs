use std::sync::{Arc};
use ratatui::layout::Rect;
use tokio::sync::Mutex;
use crate::app::App;
use crate::error::MyError;
use crate::tracing::TracingLog;
//引入组件
pub mod home;

pub mod quit;
mod tracinglog;

pub trait Component{

    fn  draw(& mut self, f:&mut ratatui::Frame<'_>,rect:Rect,app: Arc<App>)->Result<(),MyError>;
    fn  update(& mut self)->Result<(),MyError>;




}