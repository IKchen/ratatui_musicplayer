
use crate::error::MyError;
use crate:: tui::Tui;
use crate::components::home::Home;
use crate::components::Component;

pub struct App{
    should_quit:bool,
}
impl App{
    pub fn new()->Self{
        let should_quit =false;
        Self{should_quit}
    }
    pub fn run(&mut self)->Result<(),MyError>{
        let mut tui = Tui::new()?;
        let mut home=Home::new();
        tui.start().expect("初始化失败");
        tui.terminal.draw(|frame|
                     {home.draw(frame,frame.size());}
        )?;
        Ok(())
    }
}
