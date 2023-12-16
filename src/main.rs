mod error;
mod event;
mod tur;
mod components;

use crate::{
    components:: {home::Home,Component,tracinglog::TracingLog}

};


use crossterm::terminal::{self,
                          EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::event::*;
use std::io::{self,Error,Write,Stdout};
use std::panic;
use ratatui::backend::CrosstermBackend;
use crossterm::cursor::EnableBlinking;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
//自定义类型别名,避免类型名称过长
pub type CrosstermTerminal<W> = ratatui::Terminal<ratatui::backend::CrosstermBackend<W>>;
fn main(){

    let logger = TracingLog::new();
    let subscriber = tracing_subscriber::registry().with(logger);
    tracing::subscriber::set_global_default(subscriber).expect("设置订阅者失败");
   let mut tui = Tui::new(std::io::stdout()).expect("终端生成失败");

   tui.startup().expect("初始化失败");

   tui.draw().expect("绘图失败");

}
//实现终端
struct Tui<W:Write>{
    terminal:CrosstermTerminal<W>,
   // event:EventHandler
}
impl<W:Write> Tui<W>{
    pub fn new(w: W)->Result<Self,Error>{
        //新建一个终端backend，backend 需要一个实现write trait的参数，来控制终端的输入
        let backend = CrosstermBackend::new(w);
        let terminal:CrosstermTerminal<W> = ratatui::Terminal::new(backend)?;
       Ok( Self{terminal})
    }
    pub fn startup(&mut self)->Result<(),Error>{
        terminal::enable_raw_mode()?;
        crossterm::execute!(std::io::stdout(),EnterAlternateScreen,EnableMouseCapture,EnableBlinking)?;
      //钩子函数，在startup出问题时，可以执行reset
        let panic_hook = panic::take_hook();
        panic::set_hook(Box::new(move |panic| {
          Self::reset().expect("failed to reset the terminal");
          panic_hook(panic);
        }));
        //隐藏鼠标箭头
        //self.terminal.hide_cursor()?;
        //清除终端，并在下次调用draw 时，强制重绘所有ui
        self.terminal.clear()?;
        Ok(())
    }
    pub fn reset()->Result<(),Error>{
        terminal::disable_raw_mode()?;
        crossterm::execute!(io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
        Ok(())
    }
    pub fn draw(&mut self)->Result<(),Error>{
        let mut home=Home::new();
        self.terminal.draw(|frame|
            {home.draw(frame,frame.size());}
        )?;
        Ok(())
    }
    pub fn exit(&mut self) -> Result<(),Error> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}

