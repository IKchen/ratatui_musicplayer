use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use crate::action::Action;
use crate::tui::Tui;
use crate::components::quit::Quit;
use crate::components::Component;
pub struct Render{
    //pub action:Action,
    pub task:JoinHandle<()>,
    pub cancelation_token:CancellationToken,
   // pub sender:UnboundedSender<Action>,
    pub recever:UnboundedReceiver<Action>,
    pub tui:& mut Tui,
}
impl Render{
    fn new(action_receve:&mut UnboundedReceiver<Action>,tui_terminal:&mut Tui)->Self{
        let mut recever=action_receve;
        let task = tokio::spawn(async {});
        let cancelation_token=CancellationToken::new();
        let mut tui= tui_terminal;
        Self{recever,task,cancelation_token,tui}
    }
   async  fn run(&mut self){
        if let Some(rece)= self.recever.recv().await{
            match rece{
                Action::Quit=>{
                    self.tui.terminal.draw(|frame|
                        {   let mut quit=Quit::new()?;
                            quit.draw(frame,frame.size());}
                    )?;
                }
            }
        }
    }
}