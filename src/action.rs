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
    Update,
    None,
}
//事件的reactor
pub struct ActionReactor {
    pub action: Action,
    pub task: JoinHandle<()>,
    pub cancelation_token: CancellationToken,
    pub action_sender: UnboundedSender<Action>,
    pub event_receiver: Arc<Mutex<UnboundedReceiver<Event>>>,

}

impl ActionReactor {
    pub fn new(sender: UnboundedSender<Action>, receiver: UnboundedReceiver<Event>) -> Self {
        let action = Action::None;
        let task = tokio::spawn(async {});
        let cancelation_token = CancellationToken::new();
        let action_sender = sender;
        let event_receiver = Arc::new(Mutex::new(receiver));
        Self {
            action,
            task,
            cancelation_token,
            action_sender,
            event_receiver,
        }
    }

    pub  fn run(&mut self) ->JoinHandle<()>{
        let action_sender = self.action_sender.clone();
        let cancelation_token = self.cancelation_token.clone();
        let event_receiver = Arc::clone(&self.event_receiver);

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
                                    println!("Error sending action: {:?}", err);
                                } else {
                                    println!("Sent action: {:?}", Action::Quit);
                                }
                            }
                            _ => (),
                        }
                    }
                    Some(Event::Render)=>{
                        if let Err(err) = action_sender.send(Action::Render) {
                            println!("Error sending action: {:?}", err);
                        } else {
                            println!("Sent action: {:?}", Action::Render);
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