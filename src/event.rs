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
#[derive(Debug)]
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
    pub task:JoinHandle<()>,
    pub cancelation_token:CancellationToken,
    pub sender:UnboundedSender<Event>,
   // pub recever:UnboundedReceiver<Event>,
}
impl  EventHandler {
    //初始化
    pub fn new(event_sender: UnboundedSender<Event>) -> Self {
        let event = Event::None;
        let task = tokio::spawn(async {});
        let cancelation_token = CancellationToken::new();
        let sender = event_sender;
        Self { event, task, cancelation_token, sender }
    }
    //运行
    // pub fn run(&mut self,tick_rate:f64,render_rate:f64)->Result<(),MyError>{
    //     let _cancelation_token=self.cancelation_token.clone();
    //     let _event_tx = self.sender.clone();
    //     let tick_delay = std::time::Duration::from_secs_f64(1.0 / tick_rate);
    //     let render_delay = std::time::Duration::from_secs_f64(1.0 / render_rate);
    //     self.task=tokio::spawn(
    //        //避免声明周期问题，变量全部传递独立所有权，而不是引用self
    //         //在 Rust 中，你不能在同一时间既借用一个值又修改它，因为这可能导致数据竞争和不一致。
    //     //    Self::async_task(_event_tx,_cancelation_token,tick_delay,render_delay)
    //     async move{
    //             let mut reader=crossterm::event::EventStream::new();
    //             //异步执行间隔，渲染间隔
    //             let mut tick_interval = tokio::time::interval(tick_delay);
    //             let mut render_interval = tokio::time::interval(render_delay);
    //             //循环执行异步任务，直到取消异步任务
    //             loop {
    //                 let crossterm_event=reader.next().fuse();
    //                 tokio::select! {
    //                       _ = _cancelation_token.cancelled() => {break;}
    //                     maybe_event=crossterm_event=>{
    //                         match maybe_event{
    //                             Some(Ok(maybe_event))=>{
    //                                  match maybe_event {
    //                                       CrosstermEvent::Key(key) => {
    //                                         if key.kind == KeyEventKind::Press {
    //                                           _event_tx.send(Event::Key(key)).unwrap();
    //                                              println!("send the event {:?}\n",Event::Key(key));
    //                                         }
    //                                       },
    //                                       CrosstermEvent::Mouse(mouse) => {
    //                                         _event_tx.send(Event::Mouse(mouse)).unwrap();
    //                                       },
    //                                       CrosstermEvent::Resize(x, y) => {
    //                                         _event_tx.send(Event::Resize(x, y)).unwrap();
    //                                       },
    //                                       CrosstermEvent::FocusLost => {
    //                                         _event_tx.send(Event::FocusLost).unwrap();
    //                                       },
    //                                       CrosstermEvent::FocusGained => {
    //                                         _event_tx.send(Event::FocusGained).unwrap();
    //                                       },
    //                                       CrosstermEvent::Paste(s) => {
    //                                         _event_tx.send(Event::Paste(s)).unwrap();
    //                                       },
    //                                 }
    //                             }
    //                             Some(Err(_)) => {
    //                                          _event_tx.send(Event::Error).unwrap();
    //                             }
    //                             None => {},
    //
    //                         }
    //                     }
    //
    //                 }
    //             }
    //     }
    //     );
    //     Ok(())
    // }
    //接收下一事件
    /* pub async fn next(&mut self)->Option<Event>{
      let event_recv= self.recever.recv().await;
      //  println!("receiver is {:?}\n",event_recv);
        event_recv

    }*/
    pub fn close(&mut self) {
        self.cancelation_token.cancel();
    }

    // async fn async_task(_event_tx:UnboundedSender<Event>,cancellation_token: CancellationToken,tick_delay:Duration,render_delay:Duration){
    //     //读取cross term的事件流
    //     let mut reader=crossterm::event::EventStream::new();
    //     //异步执行间隔，渲染间隔
    //     let mut tick_interval = tokio::time::interval(tick_delay);
    //     let mut render_interval = tokio::time::interval(render_delay);
    //     //循环执行异步任务，直到取消异步任务
    //     loop {
    //         let crossterm_event=reader.next().fuse();
    //         tokio::select! {
    //               _ = cancellation_token.cancelled() => {break;}
    //             maybe_event=crossterm_event=>{
    //                 match maybe_event{
    //                     Some(Ok(evt))=>{
    //                          match evt {
    //                               CrosstermEvent::Key(key) => {
    //                                 if key.kind == KeyEventKind::Press {
    //                                   _event_tx.send(Event::Key(key)).unwrap();
    //                                      println!("send the event {:?}\n",Event::Key(key));
    //                                 }
    //                               },
    //                               CrosstermEvent::Mouse(mouse) => {
    //                                 _event_tx.send(Event::Mouse(mouse)).unwrap();
    //                               },
    //                               CrosstermEvent::Resize(x, y) => {
    //                                 _event_tx.send(Event::Resize(x, y)).unwrap();
    //                               },
    //                               CrosstermEvent::FocusLost => {
    //                                 _event_tx.send(Event::FocusLost).unwrap();
    //                               },
    //                               CrosstermEvent::FocusGained => {
    //                                 _event_tx.send(Event::FocusGained).unwrap();
    //                               },
    //                               CrosstermEvent::Paste(s) => {
    //                                 _event_tx.send(Event::Paste(s)).unwrap();
    //                               },
    //                         }
    //                     }
    //                     Some(Err(_)) => {
    //                                  _event_tx.send(Event::Error).unwrap();
    //                     }
    //                     None => {},
    //
    //                 }
    //             }
    //
    //         }
    //     }
    // }
    pub  fn run(&mut self, tick_rate: f64, render_rate: f64)->JoinHandle<()>{
        let cancelation_token = self.cancelation_token.clone();
        let event_tx = self.sender.clone();
        event_tx.send(Event::Render).expect("渲染事件发送失败");
      //  select_test().await?;
       tokio::spawn(Self::select_test(cancelation_token,event_tx))
       // tick_test().await?;
     //   self.task= tokio::spawn(tick_test());

    }


    pub fn handle_crossterm_event(event_tx: &UnboundedSender<Event>, maybe_event: CrosstermEvent) {
        match maybe_event {
                    CrosstermEvent::Key(key) => {
                        if key.kind == KeyEventKind::Press {
                            event_tx.send(Event::Key(key)).unwrap();
                            info!("发送事件 is {:?}\n", Event::Key(key));
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
    pub async  fn select_test(cancellation_token: CancellationToken,event_tx:UnboundedSender<Event>){
        let mut reader = crossterm::event::EventStream::new();
        let mut tick_interval = tokio::time::interval(Duration::from_secs_f64(1.0 / 4.0));
        let mut render_interval = tokio::time::interval(Duration::from_secs_f64(1.0 / 60.0));
        loop {

            let crossterm_event = reader.next().fuse();
            if cancellation_token.is_cancelled() {
                break;
            }
            tokio::select! {
                    maybe_event=crossterm_event=> {
                        match  maybe_event{
                            Some(Ok(evt))=>{
                            Self::handle_crossterm_event(&event_tx, evt);
                            }
                            Some(Err(err)) => {
                                         event_tx.send(Event::Error).ok();
                             }
                             None => {},
                        }
                    }
                     _ = tick_interval.tick() => {
                      //  event_tx.send(Event::Tick).unwrap();
                    },
                    _ = render_interval.tick() => {
                       event_tx.send(Event::Render).unwrap();
                    //    println!("render is  tick");
                    },
                }

        }

    }

}
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
                            // _ = {async move{println!("进入select")} } => {
                            //     // 处理渲染逻辑
                            //     println!("执行select");
                            // }
                    }
        println!("退出 select");
    }
    Ok(())
}

