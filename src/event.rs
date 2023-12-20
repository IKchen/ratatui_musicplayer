use std::time::Duration;
use crossterm::event::{Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent};
//use std::thread::JoinHandle;
use tokio_util::sync::CancellationToken;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};
use futures::{FutureExt, StreamExt};
use crate::error::MyError;
pub enum  Event{
    Init,
    Quit,
    Error,
    Closed,
    Tick,
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
    pub task:JoinHandle<()>,
    pub cancelation_token:CancellationToken,
    pub sender:UnboundedSender<Event>,
    pub recever:UnboundedReceiver<Event>,
}
impl  EventHandler{
    //初始化
    pub fn new()->Self{
        let event=Event::None;
        let task = tokio::spawn(async {});
        let cancelation_token=CancellationToken::new();
        let (sender,recever)=mpsc::unbounded_channel();
        Self{event,task,cancelation_token,sender,recever}
    }
    //运行
    pub fn run(&mut self,tick_rate:f64,render_rate:f64)->Result<(),MyError>{
        let _cancelation_token=self.cancelation_token.clone();
        let _event_tx = self.sender.clone();
        let tick_delay = std::time::Duration::from_secs_f64(1.0 / tick_rate);
        let render_delay = std::time::Duration::from_secs_f64(1.0 / render_rate);
        let task=tokio::spawn(
            Self::async_task(_event_tx,_cancelation_token,tick_delay,render_delay)
        );
        Ok(())
    }
    //接收下一事件
    pub async fn next(&mut self)->Option<Event>{
        self.recever.recv().await
    }
    pub fn close(&mut self){
    self.cancelation_token.cancel();
    }

    async fn async_task(_event_tx:UnboundedSender<Event>,cancellation_token: CancellationToken,tick_delay:Duration,render_delay:Duration){
        //读取crossterm的事件流
        let mut reader=crossterm::event::EventStream::new();
        //异步执行间隔，渲染间隔
        let mut tick_interval = tokio::time::interval(tick_delay);
        let mut render_interval = tokio::time::interval(render_delay);
        //循环执行异步任务，直到取消异步任务
        loop {
            let crossterm_event=reader.next().fuse();
            tokio::select! {
                  _ = cancellation_token.cancelled() => {break;}
                maybe_event=crossterm_event=>{
                    match maybe_event{
                        Some(Ok(evt))=>{
                             match evt {
                                  CrosstermEvent::Key(key) => {
                                    if key.kind == KeyEventKind::Press {
                                      _event_tx.send(Event::Key(key)).unwrap();
                                    }
                                  },
                                  CrosstermEvent::Mouse(mouse) => {
                                    _event_tx.send(Event::Mouse(mouse)).unwrap();
                                  },
                                  CrosstermEvent::Resize(x, y) => {
                                    _event_tx.send(Event::Resize(x, y)).unwrap();
                                  },
                                  CrosstermEvent::FocusLost => {
                                    _event_tx.send(Event::FocusLost).unwrap();
                                  },
                                  CrosstermEvent::FocusGained => {
                                    _event_tx.send(Event::FocusGained).unwrap();
                                  },
                                  CrosstermEvent::Paste(s) => {
                                    _event_tx.send(Event::Paste(s)).unwrap();
                                  },
                            }
                        }
                        Some(Err(_)) => {
                                     _event_tx.send(Event::Error).unwrap();
                        }
                        None => {},

                    }
                }

            }
        }
    }
}