use std::string::String;
use std::sync::{Arc};
use std::time::Duration;
use ratatui::text::Text;
use rustfft::num_complex::ComplexFloat;
use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use tokio::time::timeout;
use tracing::info;
use tracing::{Event, Subscriber};
use tracing_subscriber::layer::{Context, Layer, SubscriberExt};
use tracing_subscriber::Registry;
use crate::app::App;
//use tokio::sync::Mutex;
use crate::error::MyError;

#[derive(Debug)]
pub struct TracingLog{
    //用通信来共享内存，而不是通过共享内存来通信
    pub logs: Vec<String>,
    pub log_sender:UnboundedSender<String>,
 //   pub log_receiver:UnboundedReceiver<String>,
}

impl TracingLog {
    pub fn new() -> (Self,UnboundedReceiver<String> ){
        let logs=Vec::new();
        let (log_sender, log_receiver) = mpsc::unbounded_channel();

        ( Self{
            logs,log_sender
        },log_receiver)
    }
    //初始化日志，把tracinglog 设置为全局
    pub fn init_log(self)->Result<(),MyError>{

        let subscriber = tracing_subscriber::Registry::default().with(self);
        //println!("Subscriber initialized: {:?}", subscriber);
        // 全局的subscriber 只能有一个
        tracing::subscriber::set_global_default(subscriber).map_err(|e| {
            eprintln!("Failed to set global default subscriber: {}", e);
            MyError::InitializationError
        })?;
        //info!("初始化日志成功");
        Ok(())
    }


}
//实现layer trait
impl<S> Layer<S> for TracingLog
    where
        S: Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let log = format!("{:?}", event);
        let log_sender=self.log_sender.clone();
        // let result = tokio::spawn(async move {
        //     if let Err(err) = log_sender.send(log) {
        //         eprintln!("Failed to send log: {}", err);
        //     }
        // });
        if let Err(err) = log_sender.send(log) {
                     eprintln!("Failed to send log: {}", err);
                }  //else { println!("send sucesse!"); }

    }
}
//接收tracing产生的日志数据，并存到一个app 里面
pub  fn recv_log(log_receiver:  UnboundedReceiver<String>, app:  Arc<Mutex<App>> ) ->(JoinHandle<()>) {

    let mut log_receiver=log_receiver;

    let handle = tokio::spawn(async move {
        while let Some(log) = log_receiver.recv().await {
            // 在这里处理日志
           // let mut logs = logs_clone.lock().await;
           // logs.push(log);
           // *text.lock().await= logs.join("\n");
          //  println!("text in task is {:?}",*text.lock().await)
            app.lock().await.log.push(log)
        }
    });
    handle
}

