use std::sync::{Arc, Mutex};
use ratatui::layout::Rect;
use crate::error::MyError;
use crate::tracing::TracingLog;
//引入组件
pub mod home;

pub mod quit;
mod tracinglog;

pub trait Component{

    fn  draw(& mut self, f:&mut ratatui::Frame<'_>,rect:Rect)->Result<(),MyError>;
    fn  update(& mut self)->Result<(),MyError>;

    //获取日志信息的空白实现
    fn get_logging( & self)->TracingLog{
        TracingLog{
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }

}