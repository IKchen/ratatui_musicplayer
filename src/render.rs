
use std::sync::{Arc};
use ratatui::layout::{Constraint, Direction, Layout};
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
use crate::file::FileList;


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
    pub  fn run(& mut self,app: Arc<App>,action_sender:UnboundedSender<Action>) ->JoinHandle<Result<(), MyError>>{
        let action_receiver = Arc::clone(&self.action_receiver);
        let tui = Arc::clone(&self.tui);
        let cancelation_token = self.cancelation_token.clone();
        let shared_data_clone = self.log_text.clone();
        let mut filelist=FileList::new();


        // 将 app 参数移动到异步闭包中
        tokio::spawn(async move {
            let mut home = Home::new(shared_data_clone.lock().await.clone());
            let mut quit = Quit::new();
            let mut tracinglog= crate::components::tracinglog::TracingLogComponent::new("".to_string());
            let mut playzone=crate::components::playzone::PlayZone::new();
            //读取文件列表
            let mut filelist=FileList::new();
            filelist.load_filelist().await?;
            let filelistname=filelist.get_file_list();
            let mut filelist=crate::components::filelist::FileListComponent::new(filelistname);

            tracinglog.register_action_handler(action_sender.clone());//设置日志动作发送器，用来自动滚屏
            filelist.register_action_handler(action_sender.clone());//设置文件动作发送器，用来触发update
            while !cancelation_token.is_cancelled() {
                while let act_recv = action_receiver.lock().await.recv().await {
                    tracinglog.set_log(shared_data_clone.lock().await.clone());//读取日志
                    tracinglog.update(Some(Action::Down))?;//来一条消息，翻一条数据
                    match act_recv {
                        Some(Action::Render) => {
                            let  app_clone=Arc::clone(&app);
                            info!("收到动作: {:?}", Action::Render);

                                tui.lock().await.terminal.draw(|frame| {
                                    let layout=Layout::new(
                                        Direction::Vertical,
                                        [Constraint::Percentage(70), Constraint::Percentage(30)],
                                    )
                                        .split(frame.size());

                                    let mut sub_layout=Layout::new(
                                        Direction::Horizontal,
                                        [Constraint::Percentage(25),Constraint::Percentage(75)],
                                    ).split(layout[0]);
                                  //  home.draw(frame, layout[0],).expect("绘制图形失败");
                                    tracinglog.draw(frame,layout[1]).expect("绘制图形失败");
                                    filelist.draw(frame,sub_layout[0]).expect("绘制图形失败");
                                    playzone.draw(frame,sub_layout[1]).expect("绘制图形失败");
                                })?;

                        }
                        Some(Action::Quit) => {
                            let  app_clone=Arc::clone(&app);
                            info!("收到动作: {:?}", Action::Quit);
                            tui.lock().await.terminal.draw(|frame| {
                                quit.draw(frame, frame.size()).expect("绘制图形失败")
                            })?;
                        }
                        Some(Action::Tick) => {

                           // info!("收到动作: {:?}", Action::Tick);
                            // tui.lock().await.terminal.draw(|frame| {
                            //     home.draw(frame, frame.size(),app_clone).expect("绘制图形失败")
                            // })?;
                           // tracinglog.update(Action::Tick)?;
                           // println!("tracinglog scroll is {:?}",tracinglog.scroll);
                        }
                        Some(Action::Up)=>{
                            //tracinglog.update(Some(Action::Up))?;
                            filelist.update(Some(Action::Up))?
                        }
                        Some(Action::Down)=>{
                            filelist.update(Some(Action::Down))?
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
