
use std::sync::{Arc};
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::Duration;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Margin};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;
use tokio::task::{JoinHandle, spawn};
use tokio_util::sync::CancellationToken;
use tracing::info;
use crate::action::Action;
use crate::app::App;
use crate::components::analysis::Analysis;
use crate::components::Component;
use crate::components::home::Home;
use crate::tui::Tui;
use crate::components::quit::Quit;
use crate::error::MyError;

use crate::lyric::Lyric;
use crate::musicplayer::MusicPlayer;


pub struct Render {
    pub cancelation_token: CancellationToken,
   // pub action_receiver: UnboundedReceiver<Action>,
    pub tui: Tui,
    pub task: JoinHandle<Result<(), MyError>>,
}

impl Render {
    pub fn new( tui_terminal: Tui) -> Self {
        let cancelation_token = CancellationToken::new();
      //  let action_receiver = receiver;
        let tui =tui_terminal;
        let task = tokio::spawn(async move {Ok(())});
        Self {
            cancelation_token,
          //  action_receiver,
            tui,
            task,
        }
    }
    // pub  fn run(& mut self, app: Arc<Mutex<App>>, mut action_receiver:UnboundedReceiver<Action>) ->JoinHandle<Result<(), MyError>>{
    //     let tui = Arc::clone(&self.tui);
    //     let cancelation_token = self.cancelation_token.clone();
    //
    //     tokio::spawn(async move {
    //         while !cancelation_token.is_cancelled() {
    //             let mut app_clone =app.lock().await;
    //             if let act_recv = action_receiver.recv().await {
    //                println!("receive action is {act_recv:?}");
    //              //   println!("log is {:?}",app.lock().await.log);
    //                 app_clone.update_component();
    //                 app_clone.components.tracing_log_component.update(Some(Action::Down)).expect("滚动失败");
    //                 tui.lock().await.terminal.draw( |frame| {
    //                     app_clone.draw_component(frame);
    //                     // 其他组件的处理类似
    //                 })?;
    //
    //                 //    info!("刷新");
    //                 //     match act_recv {
    //                 //         Some(Action::Render) => {
    //                 //                 tui.lock().await.terminal.draw( |frame| {
    //                 //                     let layout=Layout::new(
    //                 //                         Direction::Vertical,
    //                 //                         [Constraint::Min(7),Constraint::Percentage(70), Constraint::Min(6)],
    //                 //                     ).split(frame.size());
    //                 //
    //                 //                     let mut sub_layout=Layout::new(
    //                 //                         Direction::Horizontal,
    //                 //                         [Constraint::Percentage(25),Constraint::Percentage(75)],
    //                 //                     ).split(layout[1]);
    //                 //                     let mut fft_layout=Layout::new(
    //                 //                         Direction::Horizontal,
    //                 //                         [Constraint::Percentage(45), Constraint::Percentage(55)],
    //                 //                     ).split(sub_layout[1]);
    //                 //                     let mut playzone_layout=Layout::new(
    //                 //                         Direction::Vertical,
    //                 //                         [Constraint::Max(3), Constraint::Min(5),Constraint::Max(3)],
    //                 //                     ).split(fft_layout[0]);
    //                 //
    //                 //                             components.play_zone.draw(frame,playzone_layout[0]).expect("绘制图形失败");
    //                 //                             components.analysis.draw(frame, fft_layout[1]).expect("绘制图形失败");
    //                 //                             components.analysis.clear_data();// 假设 Analysis 实现了 draw 方法
    //                 //                             components.tracing_log_component.draw(frame,layout[2]).expect("绘制图形失败");
    //                 //                             components.app_title.draw(frame,layout[0]).expect("绘制图形失败");
    //                 //                             components.file_list_component.draw(frame,sub_layout[0]).expect("绘制图形失败");
    //                 //                             components.music_progress.draw(frame,playzone_layout[2]).expect("绘制图形失败");
    //                 //                             components.lyric_zone.draw(frame,playzone_layout[1]).expect("绘制图形失败");
    //                 //                             其他组件的处理类似
    //                 //                })?;
    //                 //
    //                 //         }
    //                 //         Some(Action::Quit) => {
    //                 //            let  app_clone=Arc::clone(&app);
    //                 //            info!("收到动作: {:?}", Action::Quit);
    //                 //             tui.lock().await.terminal.draw(|frame| {
    //                 //                 components.quit.draw(frame,frame.size()).unwrap()
    //                 //             }).expect("绘制图形失败");
    //                 //         }
    //                 //         Some(Action::Tick) => {
    //                 //
    //                 //         }
    //                 //         Some(_) => {
    //                 //             break
    //                 //         }
    //                 //         None => break, // Channel closed
    //                 //     }
    //             }
    //         }
    //         Ok(())
    //     })
    //
    // }
    //render 中，放 动态更新的设置, reactor 中放状态的一次性设置
    pub async fn run(&mut self,app: Arc<Mutex<App>>)->Result<(),MyError>{
        let mut app=app.lock().await;
        let data=app.fft_result.lock().await.clone();
        app.components.music_progress.set_count();//开始播放计时,这里的时间要考虑不同的歌曲，计时要清零
        app.components.analysis.set_data(data);//设置fft数据
        self.tui.terminal.draw( |frame| {
                                                let layout=Layout::new(
                                                    Direction::Vertical,
                                                    [Constraint::Min(7),Constraint::Percentage(70), Constraint::Min(6)],
                                                ).split(frame.size());

                                                let mut sub_layout=Layout::new(
                                                    Direction::Horizontal,
                                                    [Constraint::Percentage(25),Constraint::Percentage(75)],
                                                ).split(layout[1]);
                                                let mut fft_layout=Layout::new(
                                                    Direction::Horizontal,
                                                    [Constraint::Percentage(45), Constraint::Percentage(55)],
                                                ).split(sub_layout[1]);
                                                let mut playzone_layout=Layout::new(
                                                    Direction::Vertical,
                                                    [Constraint::Max(3), Constraint::Min(5),Constraint::Max(3)],
                                                ).split(fft_layout[0]);
                                                match app.is_quiting {
                                                    false => {
                                                        app.components.play_zone.draw(frame,playzone_layout[0]).expect("绘制图形失败");
                                                        app.components.analysis.draw(frame, fft_layout[1]).expect("绘制图形失败");
                                                        app. components.analysis.clear_data();// 清楚fft数据
                                                        app.components.tracing_log_component.draw(frame,layout[2]).expect("绘制图形失败");
                                                        app.components.app_title.draw(frame,layout[0]).expect("绘制图形失败");
                                                        app.components.file_list_component.draw(frame,sub_layout[0]).expect("绘制图形失败");
                                                        app.components.music_progress.draw(frame,playzone_layout[2]).expect("绘制图形失败");
                                                        app.components.lyric_zone.draw(frame,playzone_layout[1]).expect("绘制图形失败");
                                                        //其他组件的处理类似
                                                        match app.sounds_list.playing_item_id {
                                                            None => {}
                                                            Some(_) => {app.components.lyric_zone.update(None).unwrap()}
                                                        }
                                                    }
                                                    true => {
                                                        app.components.quit.draw(frame,frame.size()).unwrap()
                                                    }
                                                }

                                           })?;
        Ok(())
    }
    pub fn cancel(&mut self){
        self.cancelation_token.cancel();
    }
}
