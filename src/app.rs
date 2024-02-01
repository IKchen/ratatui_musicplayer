use std::string::String;
use std::ops::DerefMut;
use std::sync::{Arc};
use std::time::Duration;
use futures::future::try_join;
use tokio::sync::{mpsc, Mutex};
use tokio::time::Instant;
use tokio::{runtime, try_join};
use tokio_util::sync::CancellationToken;
use tracing::info;
use tracing_subscriber::fmt::writer::EitherWriter::A;
use tracing_subscriber::layer::{Identity, SubscriberExt};
use crate::error::MyError;
use crate:: tui::Tui;
use crate::components::home::Home;
use crate::components::Component;
use crate::event;
use crate::action::ActionReactor;
use crate::action::Action;
use crate::render::Render;
use crate::tracing::{recv_log, TracingLog};
pub struct App{
    pub should_quit:bool,
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub tracinglog: Arc<Mutex<String>>,//日志
  //  pub log_text:String,
}
impl App{
    pub fn new()->Self{
        let should_quit =false;
        let frame_rate=60.0;
        let tick_rate=4.0;
        let mut tracinglog =Arc::new(Mutex::new(String::new()));
       // let mut log_text=String::new();
        Self{should_quit,frame_rate,tick_rate,tracinglog}
    }
//     pub async fn run(& mut self) ->Result<(),MyError>{
//
//         // 创建 EventHandler 和 ActionReactor 之间的通道
//         let (event_sender, event_receiver) = mpsc::unbounded_channel();
//
//         // 创建 ActionReactor 和 Renderer 之间的通道
//         let (action_sender, action_receiver) = mpsc::unbounded_channel();
//         let mut tui = Tui::new()?;
//         let mut home=Home::new();
//         let mut handler=event::EventHandler::new(event_sender);
//
//         //初始化tracing日志
//         let log =TracingLog::new();
//         self.tracinglog=Arc::clone(&log.logs);//获取log 中的日志数组的引用
//         let subscriber = tracing_subscriber::Registry::default().with(log);
//         println!("Subscriber initialized: {:?}", subscriber);
//         // 全局的subscriber 只能有一个
//         tracing::subscriber::set_global_default(subscriber).map_err(|e| {
//             eprintln!("Failed to set global default subscriber: {}", e);
//             MyError::InitializationError
//         })?;
//         info!("初始化成功");
//
//         // let logs=Arc::new(std::sync::Mutex::new(Vec::new()));
//         // let vec_sampler = TracingLog {
//         //     logs: Arc::clone(&logs),
//         // };
//         //
//         // let subscriber = tracing_subscriber::Registry::default()
//         //     .with(Identity::new()) // 使用默认的 Identity Layer 包装
//         //     .with(vec_sampler);
//         //
//         // println!("Subscriber initialized: {:?}", subscriber);
//         // tracing::subscriber::set_global_default(subscriber).map_err(|e| {
//         //     eprintln!("Failed to set global default subscriber: {}", e);
//         //     MyError::InitializationError
//         // })?;
//         //
//         // info!("初始化日志");
//         // info!("你好！");
//         //
//         // // 使用克隆的日志内容
//         // let text = logs.lock().unwrap().join("\n");
//         // println!("text is {:?}", text);
//         //
//         //
//         // //     tui.start().expect("初始化失败");
//         // info!("初始化成功！");
//        // println!("初始化成功！\n");
//         //把通道接收端，发送端传递给 reactor 和render
//         let mut reactor=ActionReactor::new(action_sender,event_receiver);
//         let mut render=Render::new(action_receiver, tui);
//
//         //join handle,等待异步handle 执行完任务，才退出主流程，不然主流程会执行完就退出了
//         // spawn 生成的异步task，由Tokio 的任务调度器负责调度任务队列
//         let (hanler_err, reactor_err, render_err) = tokio::join!(
//             handler.run(self.tick_rate, self.frame_rate),
//             reactor.run(),
//             render.run(Arc::new(Mutex::new(*self))),
// );
//
//     //检查各个任务的返回结果
//     //     if let Err(hanler_err) = Result::<(), _>::Err(hanler_err) {
//     //         eprintln!("Error in app run: {:?}", hanler_err);
//     //     }
//     //     if let Err(reactor_err) = Result::<(), _>::Err(reactor_err) {
//     //         eprintln!("Error in app run: {:?}", reactor_err);
//     //     }
//     //     if let Err(render_err) = Result::<(), _>::Err(render_err) {
//     //         eprintln!("Error in app run: {:?}", render_err);
//     //     }
//
//         Ok(())
//     }
}
pub async fn runner(mut app:  App) ->Result<(),MyError>{

    let logs=Arc::clone(&app.tracinglog);

    //初始化tracing日志
    //创建发送器和接收器
    let (log_sender, log_receiver) = mpsc::unbounded_channel();
    let mut log =TracingLog::new(log_sender);

    let (recv_handle) =recv_log(log_receiver,Arc::clone(&logs));

    let subscriber = tracing_subscriber::Registry::default().with(log);
    println!("Subscriber initialized: {:?}", subscriber);
    // 全局的subscriber 只能有一个
    tracing::subscriber::set_global_default(subscriber).map_err(|e| {
        eprintln!("Failed to set global default subscriber: {}", e);
        MyError::InitializationError
    })?;
    info!("初始化日志成功");

   // println!("log_text is {:?}",log_text.lock().await);
   // app.log_text=text;

    // 创建 EventHandler 和 ActionReactor 之间的通道
    let (event_sender, event_receiver) = mpsc::unbounded_channel();

    // 创建 ActionReactor 和 Renderer 之间的通道
    let (action_sender, action_receiver) = mpsc::unbounded_channel();
    let mut tui = Tui::new()?;
    let mut handler=event::EventHandler::new(event_sender);



    // let logs=Arc::new(std::sync::Mutex::new(Vec::new()));
    // let vec_sampler = TracingLog {
    //     logs: Arc::clone(&logs),
    // };
    //
    // let subscriber = tracing_subscriber::Registry::default()
    //     .with(Identity::new()) // 使用默认的 Identity Layer 包装
    //     .with(vec_sampler);
    //
    // println!("Subscriber initialized: {:?}", subscriber);
    // tracing::subscriber::set_global_default(subscriber).map_err(|e| {
    //     eprintln!("Failed to set global default subscriber: {}", e);
    //     MyError::InitializationError
    // })?;
    //
    // info!("初始化日志");
    // info!("你好！");
    //
    // // 使用克隆的日志内容
    // let text = logs.lock().unwrap().join("\n");
    // println!("text is {:?}", text);
    //
    //
    // //     tui.start().expect("初始化失败");
    // info!("初始化成功！");
    // println!("初始化成功！\n");
    //把通道接收端，发送端传递给 reactor 和render
    let mut reactor=ActionReactor::new(action_sender.clone(),event_receiver);
    let mut render=Render::new(action_receiver, tui,logs);

    //join handle,等待异步handle 执行完任务，才退出主流程，不然主流程会执行完就退出了
    // spawn 生成的异步task，由Tokio 的任务调度器负责调度任务队列
    let (hanler_err, reactor_err,
        render_err,
        _) = tokio::join!(
            handler.run(app.tick_rate, app.frame_rate),
            reactor.run(),
            render.run(Arc::new(app),action_sender.clone()),
            recv_handle,
);

    //检查各个任务的返回结果
    //     if let Err(hanler_err) = Result::<(), _>::Err(hanler_err) {
    //         eprintln!("Error in app run: {:?}", hanler_err);
    //     }
    //     if let Err(reactor_err) = Result::<(), _>::Err(reactor_err) {
    //         eprintln!("Error in app run: {:?}", reactor_err);
    //     }
    //     if let Err(render_err) = Result::<(), _>::Err(render_err) {
    //         eprintln!("Error in app run: {:?}", render_err);
    //     }

    Ok(())
}