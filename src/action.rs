use std::{fmt, string::ToString};
use std::sync::Arc;
use std::sync::mpsc::Sender;
use crossterm::event::{KeyCode, KeyEvent};
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
use tracing::info;
use crate::app::App;
use crate::components::apptitle::AppTitle;
use crate::components::Component;
use crate::components::filelist::FileListComponent;
use crate::event;

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
    Selected,
    Update,
    None,
    Down,
    Up,
    Left,
    Right,
    Processing,
    Yes,
    No,
    Start,
    Pause,
    Replay,
    Stop,
}
//事件的reactor
pub struct ActionReactor {
    pub action: Action,
    pub action_sender: Sender<(Action,Option<String>)>,
    pub last_tick_key_events_react:Vec<Action>,//tick的时候，记录上一次事件，存储的action
    pub app:Arc<Mutex<App>>,
  //  pub path_sender:Sender<String>,
}

impl ActionReactor {
    pub fn new(action_sender:Sender<(Action,Option<String>)>,app:Arc<Mutex<App>>) -> Self {
        let action = Action::None;
        let action_sender = action_sender;
        let last_tick_key_events_react=Vec::new();

        Self {
            action,
            action_sender,
            last_tick_key_events_react,
            app,
         //   path_sender
        }
    }

    pub async fn run(&mut self, event: Event){
        let action_sender = self.action_sender.clone();
        let mut last_tick_key_events_react = self.last_tick_key_events_react.clone();
                match event {
                    Event::Key(key_event) => {
                       // println!("receive key is {key_event:?}");
                        match key_event.code {
                            KeyCode::Char('q') => {
                                if let Err(err) = action_sender.send((Action::Quit,None)) {
                                    info!("Error sending action: {:?}", err);
                                } else {
                                     last_tick_key_events_react.drain(..);//存入时，先清空数组
                                    last_tick_key_events_react.push(Action::Quit);
                                    self.app.lock().await.is_quiting=true;
                                    info!("发送动作: Action::Quit", );
                                }
                            }
                            KeyCode::Char('y') => {
                                if let Err(err) = action_sender.send((Action::Yes,None)) {
                                    info!("Error sending action: {:?}", err);
                                } else {
                                    last_tick_key_events_react.drain(..);//存入时，先清空数组
                                    last_tick_key_events_react.push(Action::Quit);
                                    if self.app.lock().await.is_quiting==true{
                                        self.app.lock().await.quit_toast();
                                    }
                                    info!("发送动作: Action::Yes", );
                                }
                            }
                            KeyCode::Char('n') => {
                                if let Err(err) = action_sender.send((Action::No,None )) {
                                    info!("Error sending action: {:?}", err);
                                } else {
                                    last_tick_key_events_react.drain(..);//存入时，先清空数组
                                    last_tick_key_events_react.push(Action::Quit);
                                    if self.app.lock().await.is_quiting==true{
                                        self.app.lock().await.quit_toast();
                                    }
                                    info!("发送动作: Action::No", );
                                }
                            }
                            KeyCode::Up => {
                                if let Err(err) = action_sender.send((Action::Up,None )) {
                                    info!("Error sending action: {:?}", err);
                                    println!("Error sending action: {:?}", err);
                                } else {
                                     last_tick_key_events_react.drain(..);//清空数组
                                    last_tick_key_events_react.push(Action::Up);
                                    info!("发送动作: Action::Up");
                                    self.app.lock().await.components.file_list_component.update(Some(Action::Up)).unwrap();
                                //    println!("action is Action::up");
                                }
                            }
                            KeyCode::Down => {
                                if let Err(err) = action_sender.send((Action::Down,None )) {
                                    info!("Error sending action: {:?}", err);
                                    println!("Error sending action: {:?}", err);
                                } else {
                                     last_tick_key_events_react.drain(..);//清空数组
                                    last_tick_key_events_react.push(Action::Down);
                                    self.app.lock().await.components.file_list_component.update(Some(Action::Down)).unwrap();
                                    info!("发送动作: Aciton::Down");
                                  //  println!("发送动作 action is Action::down");
                                }
                            }
                            KeyCode::Enter => {
                                if let Err(err) = action_sender.send((Action::Selected,None)) {
                                    info!("Error sending action: {:?}", err);
                                //    println!("Error sending action: {:?}", err);
                                } else {
                                    last_tick_key_events_react.drain(..);//清空数组
                                    last_tick_key_events_react.push(Action::Selected);
                                    let selected_id=self.app.lock().await.components.file_list_component.get_selected_item_id();
                                    let playing_id=self.app.lock().await.sounds_list.playing_item_id;
                                    match selected_id {
                                        None => {info!("没有成功获取 选中数据的id")}
                                        Some(id) if Some(id) == playing_id => {
                                            // 如果选中的id和正在播放的id一致，则发送暂停动作
                                            action_sender.send((Action::Pause, None)).expect("发送动作失败");
                                        }
                                        _=>{
                                            //如果是新的id ，就准备播放
                                            self.app.lock().await.set_filelist_component_seleted_item();//设置播放音频的id
                                            self.app.lock().await.set_musicprogress_component_total_duration();//设置音频总时长
                                            self.app.lock().await.components.music_progress.reset_count();//重置音频计时
                                            self.app.lock().await.set_playing_message();//重置音频信息
                                            self.app.lock().await.set_lyric();//重置音频歌词
                                            let path=self.app.lock().await.sounds_list.get_playingsound_path();
                                            action_sender.send((Action::Start,Some(path) )).expect("发送动作失败");
                                        }
                                    }
                                   // println!("seleced_id is {selected_id:?},playing_id is {playing_id:?}");


                                    info!("发送动作: Aciton::Enter");
                                }
                            }
                            _ => (),
                        }
                    }
                    Event::Render=>{
                        if let Err(err) = action_sender.send((Action::Render,None)) {
                            println!("Error sending action: {:?}", err);
                        } else {
                            // last_tick_key_events_react.push(Action::Render);
                          //  println!("Sent action: {:?}", Action::Render);

                        }
                    }
                    Event::Tick=>{
                      //  发送上一次的action ，即重新刷新一遍动作
                      //     if let Some(last_react) = last_tick_key_events_react.last().cloned()
                      //   {
                      //       println!(" sending action: {:?}", last_react);
                      //       if let Err(err) = action_sender.send(last_react.clone()) {
                      //       //    println!("Error sending action: {:?}", err);
                      //       } else {
                      //
                      //       //    println!("Sent action: {:?}", Action::Tick);
                      //       }
                      //   }
                      //  发送tick action 去触发render tick的update 分支
                        if let Err(err) = action_sender.send((Action::Update,None)) {
                            println!("Error sending action: {:?}", err);
                        }

                    }
                    _ => (),
                }
    }

}

