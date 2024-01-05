use std::num::IntErrorKind::NegOverflow;
use std::sync::Arc;
use futures::future::ok;
use tokio::sync::Mutex;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::{JoinHandle, spawn};
use tokio_util::sync::CancellationToken;
use crate::action::Action;
use crate::components::Component;
use crate::components::home::Home;
use crate::tui::Tui;
use crate::components::quit::Quit;
use crate::error::MyError;


pub struct Render {
    pub cancelation_token: CancellationToken,
    pub action_receiver: Arc<Mutex<UnboundedReceiver<Action>>>,
    pub tui: Arc<Mutex<Tui>>,
    pub task: JoinHandle<Result<(), MyError>>,
}

impl Render {
    pub fn new(receiver: UnboundedReceiver<Action>, tui_terminal: Tui) -> Self {
        let cancelation_token = CancellationToken::new();
        let action_receiver = Arc::new(Mutex::new(receiver));
        let tui = Arc::new(Mutex::new(tui_terminal));
        let task = tokio::spawn(async move {Ok(())});
        Self {
            cancelation_token,
            action_receiver,
            tui,
            task,
        }
    }

    pub  fn run(& mut self)->JoinHandle<Result<(), MyError>>{
        let action_receiver = Arc::clone(&self.action_receiver);
        let tui = Arc::clone(&self.tui);

        let cancelation_token = self.cancelation_token.clone();

        tokio::spawn(async move {
            while !cancelation_token.is_cancelled() {
                while let act_recv = action_receiver.lock().await.recv().await {
                    match act_recv {
                        Some(Action::Render) => {
                            let mut home = Home::new();
                            println!("receive action: {:?}", Action::Render);
                            tui.lock().await.terminal.draw(|frame| {
                                home.draw(frame, frame.size()).expect("绘制图形失败")
                            })?;
                        }
                        Some(Action::Quit) => {
                            let mut quit = Quit::new();
                            println!("receive action: {:?}", Action::Quit);
                            tui.lock().await.terminal.draw(|frame| {
                                quit.draw(frame, frame.size()).expect("绘制图形失败")
                            })?;
                        }
                        Some(_) => {
                            break
                        }
                        None => break, // Channel closed
                    }
                }
            }
            Ok(())
        })

    }
    pub fn cancel(&mut self){
        self.cancelation_token.cancel();
    }
}
