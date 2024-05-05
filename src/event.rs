//use std::time::Duration;
use crossterm::event::{Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent};
//use std::thread::JoinHandle;
use tokio_util::sync::CancellationToken;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
    time::{interval,Duration},
};
use futures::{FutureExt, StreamExt};
use futures::channel::oneshot::Cancellation;
use futures::future::{ok, select};
use tokio::time::Instant;
use tracing::info;
use crate::error::MyError;
#[derive(Debug,PartialEq)]
pub enum  Event{
    Init,
    Quit,
    Error,
    Closed,
    Tick,//无事件时，渲染间隔控制
    Render,
    FocusGained,
    FocusLost,
    Paste(String),
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    None,
}
pub struct EventHandler{
    pub event:Event,
    pub cancelation_token:CancellationToken,
    pub sender:UnboundedSender<Event>,
    pub receiver:UnboundedReceiver<Event>,
}
impl  EventHandler {
    //初始化
    pub fn new() -> Self {
        let event = Event::None;
        let cancelation_token = CancellationToken::new();
        let (sender, receiver) = mpsc::unbounded_channel();
        Self { event, cancelation_token, sender ,receiver}
    }

    pub fn close(&mut self) {
        self.cancelation_token.cancel();
    }

    pub  fn run(&mut self)->JoinHandle<()>{
        let cancelation_token = self.cancelation_token.clone();
        let event_tx = self.sender.clone();
        tokio::spawn(select_task(cancelation_token,event_tx))
    }
    pub async fn next(&mut self) -> Option<Event> {
        self.receiver.recv().await
    }

}
pub async  fn select_task(cancellation_token: CancellationToken,event_tx:UnboundedSender<Event>){
    let mut reader = crossterm::event::EventStream::new();
    let mut tick_interval = tokio::time::interval(Duration::from_secs_f64(1.0 / 4.0));
    let mut render_interval = tokio::time::interval(Duration::from_secs_f64(1.0 ));
    loop {

        let crossterm_event = reader.next().fuse();
        if cancellation_token.is_cancelled() {
            break;
        }
        tokio::select! {
                    maybe_event=crossterm_event=> {
                        match  maybe_event{
                            Some(Ok(evt))=>{
                            handle_crossterm_event(&event_tx, evt);
                            }
                            Some(Err(err)) => {
                            event_tx.send(Event::Error).ok();
                             }
                             None => {},
                        }
                    }
                     _ = tick_interval.tick() => {
                    //    event_tx.send(Event::Tick).unwrap();
                    },
                    _ = render_interval.tick() => {
                    //   event_tx.send(Event::Render).unwrap();
                    //    println!("render is  tick");
                    },
            }

    }

}
//处理crossterm事件
pub fn handle_crossterm_event(event_tx: &UnboundedSender<Event>, maybe_event: CrosstermEvent) {
    match maybe_event {
        CrosstermEvent::Key(key) => {
            if key.kind == KeyEventKind::Press {
                event_tx.send(Event::Key(key)).unwrap();
              //  info!("发送事件 is {:?}\n", Event::Key(key));
             //   println!("发送事件 is {:?}", Event::Key(key));
            }
        },
        CrosstermEvent::Mouse(mouse) => {
            event_tx.send(Event::Mouse(mouse)).unwrap();
        },
        CrosstermEvent::Resize(x, y) => {
            event_tx.send(Event::Resize(x, y)).unwrap();
        },
        CrosstermEvent::FocusLost => {
            event_tx.send(Event::FocusLost).unwrap();
        },
        CrosstermEvent::FocusGained => {
            event_tx.send(Event::FocusGained).unwrap();
        },
        CrosstermEvent::Paste(s) => {
            event_tx.send(Event::Paste(s)).unwrap();
        },
        _ => {}
    }

}
//测试用
pub async  fn tick_test()->Result<(), MyError>{
    let mut tick_interval = tokio::time::interval(Duration::from_secs_f64(1.0 / 4.0));
    let mut render_interval = tokio::time::interval(Duration::from_secs_f64(1.0 / 60.0));
    let mut cancelation_token = CancellationToken::new();
    let start_time = Instant::now();
    loop {
        info!("进入循环  is {:?}",render_interval);
        let current_time = Instant::now();
        //  let crossterm_event =  futures::StreamExt::next(&mut reader).fuse();
        if cancelation_token.is_cancelled() {
            info!("is_cancelled is cancelled");
            break;
        }
        let elapsed_time = current_time - start_time;
        if elapsed_time >= Duration::from_secs(5) {
            info!("is_cancelled is 准备取消1" );
            cancelation_token.cancel();

        }
        tokio::select! {
                        _ = tick_interval.tick()=> {
                                    println!("tick_interval is {:?}", tick_interval);

                        }
                        _ = render_interval.tick()=> {
                            // 处理渲染逻辑
                                    println!("render_interval is {:?}", render_interval);
                        }

                    }
        println!("退出 select");
    }
    Ok(())
}

#[cfg(test)]
mod tests{
    use std::time::Duration;
    use crate::event::{Event, EventHandler};
    use tokio::test;
    use tokio::time::timeout;
    use crate::event;

    // 引入超时处理
    //测试通道是否畅通
    #[tokio::test]
    async fn test_event_handle(){
        let mut handle =EventHandler::new();
        tokio::spawn(async move {
            handle.sender.send(Event::Tick).unwrap();
        }).await.unwrap();
        let reciever=handle.receiver.recv().await.unwrap();
        assert_eq!(Event::Tick, reciever);
    }
    //测试run函数
    #[tokio::test]
    async fn test_event_handler_run() {
        // 创建一个新的事件处理器实例
        let mut handler = EventHandler::new();
        // 运行事件处理器的 run 方法，并在另一个任务中执行
        let handle = handler.run();

        // 等待接收第一个事件
        let received_event = timeout(Duration::from_secs(10), handler.next()).await;

        // 确保事件被接收到，并且是正确的事件类型
        assert!(received_event.is_ok(), "没有在预期时间内接收到事件");
        let event = received_event.unwrap();
        assert!(event.is_some(), "接收到的事件是 None");
    //    assert_eq!(event.unwrap(), event::Event::Render, "接收到的事件不是 Event::Render");

        // 关闭 handle 以避免在测试结束后继续运行
        handler.close();
        handle.await.expect("事件处理任务运行失败");
    }
}