use std::{fmt, string::ToString};
use tokio::sync::mpsc;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use serde::{
    de::{self, Deserializer, Visitor},
    Deserialize, Serialize,
};
use crate::event::Event;

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
pub struct ActionReactor{
    pub action:Action,
    pub task:JoinHandle<()>,
    pub cancelation_token:CancellationToken,
    pub sender:UnboundedSender<Action>,
    pub recever:UnboundedReceiver<Action>,
}
impl ActionReactor{
    pub fn new()->Self{
        let action=Action::None;
        let task = tokio::spawn(async {});
        let cancelation_token=CancellationToken::new();
        let (sender,recever)=mpsc::unbounded_channel();
        Self{action,task,cancelation_token,sender,recever}
    }
    pub fn run(&mut self,event:Event){
        match event {
            Event::Quit=>{
                self.sender.send(Action::Quit)?
            }
            _=>{}
        }
    }
}