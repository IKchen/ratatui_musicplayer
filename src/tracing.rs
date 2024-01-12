use std::sync::{Arc, Mutex};
use log::info;
use tracing::{Event, Subscriber};
use tracing_subscriber::layer::{Context, Layer, SubscriberExt};
use tracing_subscriber::Registry;
use crate::error::MyError;

#[derive(Debug,Clone)]
pub struct TracingLog{
    pub logs: Arc<Mutex<Vec<String>>>,
}

impl TracingLog {
    pub fn new() -> Self {
        Self{
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn initialize_logging(&mut self) -> Result<(), MyError> {
        let logs_shared = self.logs.clone(); // 对self的log 进行解引用
        let subscriber = tracing_subscriber::Registry::default().with(TracingLog {
            logs: logs_shared.clone(),
        });
        println!("Subscriber initialized: {:?}", subscriber);
        // 全局的subscriber 只能有一个
        tracing::subscriber::set_global_default(subscriber).map_err(|e| {
            eprintln!("Failed to set global default subscriber: {}", e);
            MyError::InitializationError
        })?;
        info!("初始化成功");
        self.logs=logs_shared;
        Ok(())
    }
    //获取日志中的log 数据
    pub fn get_log(&mut self)-> Arc<Mutex<Vec<String>>>{
       let log= self.logs.clone();
        log
    }

}

impl<S> Layer<S> for TracingLog
    where
        S: Subscriber,
{
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let mut logs = self.logs.lock().unwrap();
        let log = format!("{:?}", event);
        logs.push(log);
    }
}
