use std::num::IntErrorKind::NegOverflow;
use std::sync::{Arc};
use futures::future::ok;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;
use tokio::task::{JoinHandle, spawn};
use tokio_util::sync::CancellationToken;
use tracing::info;
use crate::action::Action;
use crate::app::App;
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
    pub log_text: Arc<Mutex<String>>,
}

impl Render {
    pub fn new(receiver: UnboundedReceiver<Action>, tui_terminal: Tui, log_text: Arc<Mutex<String>>) -> Self {
        let cancelation_token = CancellationToken::new();
        let action_receiver = Arc::new(Mutex::new(receiver));
        let tui = Arc::new(Mutex::new(tui_terminal));
        let task = tokio::spawn(async move {Ok(())});
        let log_text =log_text;
        Self {
            cancelation_token,
            action_receiver,
            tui,
            task,
            log_text
        }
    }
    //引用app的时候，render的run函数 无法知道 app 引用的生命周期是否能覆盖run的生命周期，只有在app run 中调用 render run才能保证app的生命周期
    //大于run（但是编译器不知道，也不能限制只能在app run 中调用 render run ）
    pub  fn run(& mut self,app: Arc<App>) ->JoinHandle<Result<(), MyError>>{
        let action_receiver = Arc::clone(&self.action_receiver);
        let tui = Arc::clone(&self.tui);
        let cancelation_token = self.cancelation_token.clone();
        let shared_data_clone = self.log_text.clone();
        // 将 app 参数移动到异步闭包中
        tokio::spawn(async move {
            while !cancelation_token.is_cancelled() {
                while let act_recv = action_receiver.lock().await.recv().await {
                    match act_recv {
                        Some(Action::Render) => {
                            let log_text = shared_data_clone.lock().await.clone();
                            let mut home = Home::new(log_text);
                            let mut quit = Quit::new();
                            let app_clone=Arc::clone(&app);

                            info!("receive action: {:?}", Action::Render);
                         //   println!("开始绘制");
                                tui.lock().await.terminal.draw(|frame| {
                                   // println!("正在绘制");
                                    home.draw(frame, frame.size(),app_clone).expect("绘制图形失败")
                                })?;

                        }
                        Some(Action::Quit) => {
                            let mut quit = Quit::new();
                            let app_clone=Arc::clone(&app);
                            info!("receive action: {:?}", Action::Quit);
                            tui.lock().await.terminal.draw(|frame| {
                                quit.draw(frame, frame.size(),app_clone).expect("绘制图形失败")
                            })?;
                        }
                        Some(Action::Tick) => {
                            let mut quit = Quit::new();
                            let app_clone=Arc::clone(&app);
                            info!("receive action: {:?}", Action::Quit);
                            tui.lock().await.terminal.draw(|frame| {
                                quit.draw(frame, frame.size(),app_clone).expect("绘制图形失败")
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
