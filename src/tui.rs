use ratatui::backend::{CrosstermBackend as Backend, CrosstermBackend};
use ratatui::Terminal;
use crate::{error::MyError,event};
use std::{thread,io};
use crossterm::terminal::{self, EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::cursor;
use crossterm::event::DisableMouseCapture;
use std::panic;
//use std::io::*;
 pub struct Tui{
    //终端
    pub terminal:Terminal<Backend<std::io::Stdout>>,
    //帧率

 }

impl Tui{
    //实例化
    pub fn new()->Result<Self,MyError>{
        let terminal=ratatui::Terminal::new(Backend::new(std::io::stdout()))?;

        Ok(Self{terminal})
    }
    //启动
    pub fn start(&mut self)->Result<(),MyError>{
        crossterm::terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stdout(), EnterAlternateScreen, cursor::Hide)?;
        //钩子函数，在startup出问题时，可以执行reset
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
            Self::reset().expect("failed to reset the terminal");
            panic_hook(panic);
        }));
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
    pub fn reset()->Result<(),MyError>{
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }
}