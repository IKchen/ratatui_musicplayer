
use crate::error::MyError;
use crate:: tui::Tui;
use crate::components::home::Home;
use crate::components::Component;
use crate::event;
use crate::action::ActionReactor;
use crate::action::Action;
use crate::render::Render;

pub struct App{
    pub should_quit:bool,
    pub tick_rate: f64,
    pub frame_rate: f64,
}
impl App{
    pub fn new()->Self{
        let should_quit =false;
        let frame_rate=60.0;
        let tick_rate=60.0;
        Self{should_quit,frame_rate,tick_rate}
    }
    pub async fn run(&mut self)->Result<(),MyError>{
        let mut tui = Tui::new()?;
        let mut home=Home::new();
        let mut handler=event::EventHandler::new();
        let mut reactor=ActionReactor::new();
        let mut render=Render::new(& mut reactor.recever,& mut tui);
        //运行事件handler
        handler.run(self.tick_rate,self.frame_rate)?;
        //循环判断事件
        loop {
            if let Some(eve)=handler.next().await{
                reactor.run(eve);
                break
            }
        }
        loop{
            render.run().await?;
        }
        tui.start().expect("初始化失败");
        tui.terminal.draw(|frame|
                     {home.draw(frame,frame.size());}
        )?;
        Ok(())
    }
}
