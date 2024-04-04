use std::{fmt, string::ToString};
use std::sync::Arc;
use crossterm::event::{KeyCode, KeyEvent};
use futures::future::ok;
use tokio::sync::{mpsc, MutexGuard,Mutex};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use serde::{
    de::{self, Deserializer, Visitor},
    Deserialize, Serialize,
};
use crate::event::Event;
use futures::{FutureExt, StreamExt};
use tracing::info;

//// ANCHOR: action_enum
#[derive(Debug, Clone, PartialEq, Eq, Serialize,  Deserialize)]
pub enum Action {
    Tick,
    Render,
    Resize(u16, u16),
    Suspend,
    Resume,
    Quit,
    Refresh,
    Error(String),
    Help,
    ToggleShowHelp,
    ScheduleIncrement,
    ScheduleDecrement,
    Increment(usize),
    Decrement(usize),
    CompleteInput(String),
    EnterNormal,
    EnterInsert,
    EnterProcessing,
    ExitProcessing,
    Selected,
    Update,
    None,
    Down,
    Up,
    Left,
    Right,
    Processing,
}
//事件的reactor
pub struct ActionReactor {
    pub action: Action,
    pub task: JoinHandle<()>,
    pub cancelation_token: CancellationToken,
    pub action_sender: UnboundedSender<Action>,
    pub event_receiver: Arc<Mutex<UnboundedReceiver<Event>>>,
    pub last_tick_key_events_react:Vec<Action>,//tick的时候，记录上一次事件，存储的action

}

impl ActionReactor {
    pub fn new(sender: UnboundedSender<Action>, receiver: UnboundedReceiver<Event>) -> Self {
        let action = Action::None;
        let task = tokio::spawn(async {});
        let cancelation_token = CancellationToken::new();
        let action_sender = sender;
        let event_receiver = Arc::new(Mutex::new(receiver));
        let last_tick_key_events_react=Vec::new();
        Self {
            action,
            task,
            cancelation_token,
            action_sender,
            event_receiver,
            last_tick_key_events_react,
        }
    }

    pub  fn run(&mut self) ->JoinHandle<()>{
        let action_sender = self.action_sender.clone();
        let cancelation_token = self.cancelation_token.clone();
        let event_receiver = Arc::clone(&self.event_receiver);
        let mut last_tick_key_events_react = self.last_tick_key_events_react.clone();

        tokio::spawn(async move {
            loop {
                //为什么要用if？ 如果把cancelation的判读设置成异步任务，如果其他任务频繁执行，可能导致取消任务永远无法执行
                if cancelation_token.is_cancelled() {
                    break;
                }

                // Locking the tokio::sync::Mutex
                let mut guard =  event_receiver.lock().await;
            /*    {
                    Ok(guard) => guard,
                    Err(err) => {
                        println!("Error locking event_receiver: {:?}", err);
                        // Handle the error as needed
                        break;
                    }
                }*/

                match guard.recv().await {
                    Some(Event::Key(key_event)) => {
                        match key_event.code {
                            KeyCode::Char('q') => {
                                if let Err(err) = action_sender.send(Action::Quit) {
                                    info!("Error sending action: {:?}", err);
                                } else {
                                     last_tick_key_events_react.drain(..);//存入时，先清空数组
                                    last_tick_key_events_react.push(Action::Quit);
                                    info!("发送动作: Action::Quit", );
                                }
                            }
                            KeyCode::Char('i') => {
                                if let Err(err) = action_sender.send(Action::Up) {
                                    info!("Error sending action: {:?}", err);
                                } else {
                                     last_tick_key_events_react.drain(..);//清空数组
                                    last_tick_key_events_react.push(Action::Up);
                                    info!("发送动作: Action::Up");
                                }
                            }
                            KeyCode::Char('k') => {
                                if let Err(err) = action_sender.send(Action::Down) {
                                    info!("Error sending action: {:?}", err);
                                } else {
                                     last_tick_key_events_react.drain(..);//清空数组
                                    last_tick_key_events_react.push(Action::Down);
                                    info!("发送动作: Aciton::Down");
                                }
                            }
                            KeyCode::Enter => {
                                if let Err(err) = action_sender.send(Action::Selected) {
                                    info!("Error sending action: {:?}", err);

                                } else {
                                    last_tick_key_events_react.drain(..);//清空数组
                                    last_tick_key_events_react.push(Action::Down);
                                    info!("发送动作: Aciton::Down");
                                }
                            }
                            _ => (),
                        }
                    }
                    Some(Event::Render)=>{
                        if let Err(err) = action_sender.send(Action::Render) {
                            println!("Error sending action: {:?}", err);
                        } else {
                            // last_tick_key_events_react.push(Action::Render);
                          //   println!("Sent action: {:?}", Action::Render);
                        }
                    }
                    Some(Event::Tick)=>{
                      //  发送上一次的action ，即重新刷新一遍动作
                      //     if let Some(last_react) = last_tick_key_events_react.last().cloned()
                      //   {
                      //       println!(" sending action: {:?}", last_react);
                      //       if let Err(err) = action_sender.send(last_react.clone()) {
                      //       //    println!("Error sending action: {:?}", err);
                      //       } else {
                      //
                      //       //    println!("Sent action: {:?}", Action::Tick);
                      //       }
                      //   }
                      //  发送tick action 去触发render tick的update 分支
                        if let Err(err) = action_sender.send(Action::Update) {
                            println!("Error sending action: {:?}", err);
                        }

                    }
                    None => {
                        // Handle channel closure
                        break;
                    }
                    _ => (),
                }
            }
        })
    }
}