use crossterm::event::*;
//use std::thread::JoinHandle;
use tokio_util::sync::CancellationToken;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver, UnboundedSender},
    task::JoinHandle,
};

use crate::error::MyError;
pub enum  Event{
    Quit,
    Resize(u64,u64),
    KeyEvent(KeyEvent),
    MouseEvent,
    Tick,
    None,
}
pub struct EventHandler{
    pub event:Event,
    pub task:JoinHandle<()>,
    pub cancelation_token:CancellationToken,

}
impl  EventHandler{
    pub fn new()->Self{
        let event=Event::None;
        let task = tokio::spawn(async {});
        let cancelation_token=CancellationToken::new();
        Self{event,task,cancelation_token}
    }
    pub fn run()->Result<(),MyError>{
    todo!()
    }
    pub fn next()->Result<Event,MyError>{
    todo!()
    }
    pub fn close()->Result<Event,MyError>{
    todo!()
    }
}