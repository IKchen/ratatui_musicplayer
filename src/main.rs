use crossterm::terminal::{self, 
    EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::event::*;
use std::io::{self,Error,Write,Stdout};
use std::panic;
use ratatui::backend::CrosstermBackend;

use crossterm::cursor::EnableBlinking;

//自定义类型别名,避免类型名称过长
pub type CrosstermTerminal<W> = ratatui::Terminal<ratatui::backend::CrosstermBackend<W>>;
fn main(){
   let mut tui = Tui::new(std::io::stdout()).expect("终端生成失败");
   tui.startup().expect("初始化失败");
   let app=App::new();
   tui.draw().expect("绘图失败");

}
//实现终端
struct Tui<W:Write>{
    terminal:CrosstermTerminal<W>,
    event:EventHandler
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
        self.terminal.draw(|frame|render(frame))?;
        Ok(())
    }
    pub fn exit(&mut self) -> Result<(),Error> {
        Self::reset()?;
        self.terminal.show_cursor()?;
        Ok(())
    }
}
//ui 渲染
use ratatui::layout::{Layout,Direction::{*},Constraint,Rect};
use ratatui::widgets::*;
use ratatui::style::Stylize;
//pub type Frame<'a>=ratatui::Frame<'a, ratatui::backend::CrosstermBackend<std::io::Stdout>>;
pub fn render<W:Write>(  f:&mut ratatui::Frame<'_, ratatui::backend::CrosstermBackend<W>>){
    let layout=Layout::default()
    .direction(Vertical)
    .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
    .split(Rect::new(0, 0, 10, 10));
    f.render_widget(Paragraph::new("内容1")
        .block( Block::new()
        .title("标题1").red()
        .borders(Borders::ALL)).blue().on_green(), layout[0]);
    f.render_widget(Paragraph::new("内容2")
        .block( Block::new()
        .title("标题2").red()
        .borders(Borders::ALL)), layout[1]);
    
}
//app 数据结构
struct App{
    should_quit:bool,
}
impl App{
    pub fn new()->Self{
        let should_quit=false;
        Self{ should_quit}
    }
    pub fn quit(&mut self){
        self.should_quit=true;
    }
}
//事件枚举
enum Event{
    TimeEvent,
    KeyEvent,
    MouseEvent,
    Resize(u16,u16),
}
//实现事件控制
use std::{thread,time::{Duration,Instant},sync::mpsc::{self,RecvError}}};
struct EventHandler{
 
}