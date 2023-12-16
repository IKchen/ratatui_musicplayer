use ratatui::backend::{CrosstermBackend as Backend, CrosstermBackend};
use ratatui::Terminal;
use crate::{error::MyError,event};
use std::{thread,io};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor;
use crossterm::event::DisableMouseCapture;
//use std::io::*;
 pub struct Tui{
    //终端
    pub terminal:Terminal<Backend<std::io::Stdout>>,
    //帧率
    pub frame_rate: f64,
    pub tick_rate: f64,
 }

impl Tui{
    //实例化
    pub fn new()->Result<Self,MyError>{
        let terminal=ratatui::Terminal::new(Backend::new(std::io::stdout()))?;
        let frame_rate=60.0;
        let tick_rate=60.0;
        Ok(Self{terminal,frame_rate,tick_rate})
    }
    //启动
    pub fn start(&mut self)->Result<(),MyError>{
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stdout(), EnterAlternateScreen, cursor::Hide)?;
        let handler=event::EventHandler::new();
        Ok(())
    }
    //退出
    pub fn exit(&mut self)->Result<(),MyError>{
        self.cancel()?;
        crossterm::terminal::disable_raw_mode()?;
        crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }
    //获取下一事件
    pub fn next(&mut self)->Result<(),MyError>{
        todo!()
    }
    //取消任务
    pub fn cancel(&mut self)->Result<(),MyError>{

        todo!()
    }
}