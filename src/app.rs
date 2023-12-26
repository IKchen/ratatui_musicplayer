use std::sync::{Arc, Mutex};
use log::info;
use tokio::sync::mpsc;
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

        // 创建 EventHandler 和 ActionReactor 之间的通道
        let (event_sender, event_receiver) = mpsc::unbounded_channel();

        // 创建 ActionReactor 和 Renderer 之间的通道
        let (action_sender, action_receiver) = mpsc::unbounded_channel();
        let mut tui = Tui::new()?;
        let mut home=Home::new();
        let mut handler=event::EventHandler::new(event_sender);




        tui.start().expect("初始化失败");
        info!("初始化成功！");
        println!("初始化成功！\n");
        //把通道传递给 reactor 和render
        let mut reactor=ActionReactor::new(action_sender,event_receiver);
        let mut render=Render::new(action_receiver, tui);

        //运行事件handler
        handler.run(self.tick_rate,self.frame_rate)?;
        //循环判断事件
        reactor.run();
        render.run().await?;

        Ok(())
    }
}
