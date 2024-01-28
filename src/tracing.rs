use std::string::String;
use std::sync::{Arc};
use std::time::Duration;
use ratatui::text::Text;
use tokio::sync::{mpsc, Mutex};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use tokio::time::timeout;
use tracing::info;
use tracing::{Event, Subscriber};
use tracing_subscriber::layer::{Context, Layer, SubscriberExt};
use tracing_subscriber::Registry;
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
    pub fn new(log_sender:UnboundedSender<String>) -> Self {
        let logs=Vec::new();
     //   let (log_sender, log_receiver) = mpsc::unbounded_channel();

        Self{
            logs,log_sender
        }
    }

    // pub fn initialize_logging(self) -> Result<(), MyError> {
    //     let logs_shared = Arc::clone(&self); // 对self的log 进行解引用
    //     let subscriber = tracing_subscriber::Registry::default().with(logs_shared);
    //     println!("Subscriber initialized: {:?}", subscriber);
    //     // 全局的subscriber 只能有一个
    //     tracing::subscriber::set_global_default(subscriber).map_err(|e| {
    //         eprintln!("Failed to set global default subscriber: {}", e);
    //         MyError::InitializationError
    //     })?;
    //     info!("初始化成功");
    //
    //     Ok(())
    // }
   // 获取日志中的log 数据
   //  pub fn get_log(& self)-> Vec<String>{
   //     let log= Arc::clone(&self.logs).lock().unwrap();
   //     let logs=(*log).clone();
   //      logs
   //  }

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
pub  fn recv_log(log_receiver:  UnboundedReceiver<String>, text: Arc<Mutex<String>> ) ->(JoinHandle<()>) {
    //let text = Arc::new(Mutex::new(String::new()));
    let logs = Arc::new(Mutex::new(Vec::new()));
    let mut log_receiver=log_receiver;
    let text_clone = Arc::clone(&text);
    let logs_clone = Arc::clone(&logs);

    let handle = tokio::spawn(async move {
        while let Some(log) = log_receiver.recv().await {
            // 在这里处理日志
            let mut logs = logs_clone.lock().await;
            logs.push(log);
        //    let mut text = text.lock().unwrap();
            *text.lock().await= logs.join("\n");
          //  println!("text in task is {:?}",*text.lock().await)
        }
    });

    //handle.await.unwrap(); // 等待异步任务完成

    // let x=text.lock().unwrap().clone();
    // (handle,x)
    handle
    // 设置超时时间为5秒，可以根据实际需求调整
    // let timeout_duration = Duration::from_secs(5);
    // let result = timeout(timeout_duration, handle).await;
    //
    // match result {
    //     Ok(_) => {
    //         // 异步任务完成，返回最终的text值
    //         let x = text.lock().await.clone();
    //         x
    //     }
    //     Err(_) => {
    //         // 超时未收到消息，返回当前的text值
    //         let x = text.lock().await.clone();
    //         x
    //     }
    // }
}