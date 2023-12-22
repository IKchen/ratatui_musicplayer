use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use crate::action::Action;
use crate::tui::Tui;
use crate::components::quit::Quit;
use crate::components::Component;
use crate::error::MyError;

pub struct Render{
    //pub action:Action,
    pub task:JoinHandle<()>,
    pub cancelation_token:CancellationToken,
   // pub sender:UnboundedSender<Action>,
   pub recever: Arc<Mutex<UnboundedReceiver<Action>>>,
    pub tui:  Tui,
}
impl Render{
    pub fn new(action_receve:Arc<Mutex<UnboundedReceiver<Action>>>,tui_terminal: Tui)->Self{
        let mut recever=action_receve;
        let task = tokio::spawn(async {});
        let cancelation_token=CancellationToken::new();
        let mut tui= tui_terminal;
        Self{recever,task,cancelation_token,tui}
    }
   pub async fn run(&mut self)->Result<(),MyError>{
       let mut receiver = self.recever.lock().unwrap();
        if let Some(rece)= receiver.recv( ).await{
            match rece{
                Action::Quit=>{
                    self.tui.terminal.draw(|frame|
                        {   let mut quit=Quit::new();
                            quit.draw(frame,frame.size());}
                    )?;
                }
                _=>{}
            }
        }
       Ok(())
    }
}