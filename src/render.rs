use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::{JoinHandle, spawn};
use tokio_util::sync::CancellationToken;
use crate::action::Action;
use crate::components::Component;
use crate::tui::Tui;
use crate::components::quit::Quit;
use crate::error::MyError;


pub struct Render {
    pub cancelation_token: CancellationToken,
    pub action_receiver: Arc<Mutex<UnboundedReceiver<Action>>>,
    pub tui: Arc<Mutex<Tui>>,
}

impl Render {
    pub fn new(receiver: UnboundedReceiver<Action>, tui_terminal: Tui) -> Self {
        let cancelation_token = CancellationToken::new();
        let action_receiver = Arc::new(Mutex::new(receiver));
        let tui = Arc::new(Mutex::new(tui_terminal));
        Self {
            cancelation_token,
            action_receiver,
            tui,
        }
    }

    pub fn run(self) -> JoinHandle<Result<(), MyError>> {
        let action_receiver = Arc::clone(&self.action_receiver);
        let tui = Arc::clone(&self.tui);

        let cancelation_token = self.cancelation_token.clone();

        spawn(async move {
            while !cancelation_token.is_cancelled() {
                match action_receiver.lock().await.recv().await {
                    Some(Action::Quit) => {
                        let mut quit=Quit::new();
                        println!("receive action: {:?}", Action::Quit);
                        tui.lock().await.terminal.draw(|frame|{
                            quit.draw(frame,frame.size()).unwrap()
                        })?;
                    }
                    Some(_) => {
                        // Handle other actions
                    }
                    None => break, // Channel closed
                }
            }
            Ok(())
        })
    }
}
