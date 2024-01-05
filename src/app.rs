use std::sync::{Arc, Mutex};
use std::time::Duration;
use futures::future::try_join;
use log::info;
use tokio::sync::mpsc;
use tokio::time::Instant;
use tokio::{runtime, try_join};
use tokio_util::sync::CancellationToken;
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
        let tick_rate=4.0;
        Self{should_quit,frame_rate,tick_rate}
    }
    pub async fn run(&mut self) ->Result<(),MyError>{

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

        //join handle,等待异步handle 执行完任务，才退出主流程，不然主流程会执行完就退出了
        // spawn 生成的异步task，由Tokio 的任务调度器负责调度任务队列
        let (hanler_err, reactor_err, render_err) = tokio::join!(
            handler.run(self.tick_rate, self.frame_rate),
            reactor.run(),
            render.run(),
);

    //检查各个任务的返回结果
        if let Err(hanler_err) = Result::<(), _>::Err(hanler_err) {
            eprintln!("Error in app run: {:?}", hanler_err);
        }
        if let Err(reactor_err) = Result::<(), _>::Err(reactor_err) {
            eprintln!("Error in app run: {:?}", reactor_err);
        }
        if let Err(render_err) = Result::<(), _>::Err(render_err) {
            eprintln!("Error in app run: {:?}", render_err);
        }

        Ok(())
    }
}
