use std::string::String;
use std::ops::DerefMut;
use std::sync::{Arc};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;
use futures::channel::mpsc::unbounded;
use futures::future::try_join;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout};
use tokio::sync::{mpsc, Mutex};
use tokio::time::Instant;
use tokio::{runtime, try_join};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::info;
use tracing_subscriber::fmt::writer::EitherWriter::A;
use tracing_subscriber::layer::{Identity, SubscriberExt};
use crate::error::MyError;
use crate:: tui::Tui;
use crate::components::home::Home;
use crate::components::{Component, Components};
use crate::{action, app, event, render};
use crate::action::ActionReactor;
use crate::action::Action;
use crate::fft::{FFTController, get_fft_result};

use crate::lyric::LyricController;
use crate::musicplayer::MusicPlayer;
use crate::render::Render;
use crate::sounds::SoundsList;
use crate::tracing::{ recv_log, TracingLog};

pub struct App{
    pub should_quit:bool,
    pub tick_rate: f64,
    pub frame_rate: f64,
    pub log: Vec<String>,//日志
    pub sounds_list: SoundsList,
    pub fft_result:Arc<Mutex<Vec<(String,u64)>>>,
    pub components: Components,
    pub is_quiting:bool,//判断是否渲染退出弹窗
}
impl App{
    pub fn new()->Self{
        let should_quit =false;
        let frame_rate=60.0;
        let tick_rate=4.0;
        let mut log=vec![String::new()];
        let mut sounds_list=SoundsList::set_path("music".to_string());
        let mut fft_result=Arc::new(Mutex::new(
            vec![
                ("B1".to_string(), 0u64),
                ("B2".to_string(), 0u64),
                ("B3".to_string(), 0u64),
                ("B4".to_string(), 0u64),
                ("B5".to_string(), 0u64),
                ("B6".to_string(), 0u64),
                ("B7".to_string(), 0u64),
                ("B8".to_string(), 0u64),
                ("B9".to_string(), 0u64),
                ("B10".to_string(), 0u64),
                ("B11".to_string(), 0u64),
                ("B12".to_string(), 0u64),]
        ));
        let components= crate::components::Components::new();
        Self{should_quit,frame_rate,tick_rate,sounds_list,fft_result,log,components,is_quiting:false}
    }
    //初始化组件
    pub fn init_component(&mut self, action_sender: Sender<(Action,Option<String>)>){
        // self.components.play_zone.register_action_handler(action_sender.clone());
        // self.components.file_list_component.register_action_handler(action_sender.clone());
        // self.components.analysis.register_action_handler(action_sender.clone());
        // self.components.app_title.register_action_handler(action_sender.clone());
        // self.components.lyric_zone.register_action_handler(action_sender.clone());
        // self.components.music_progress.register_action_handler(action_sender.clone());
        // self.components.tracing_log_component.register_action_handler(action_sender.clone());
        // self.components.quit.register_action_handler(action_sender.clone());
        self.components.file_list_component.set_file_list(self.sounds_list.clone());

    }
    //绘制组件
    pub fn draw_component(&mut self, frame: &mut Frame){
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

        self.components.play_zone.draw(frame,playzone_layout[0]).expect("绘制图形失败");
        self.components.analysis.draw(frame, fft_layout[1]).expect("绘制图形失败");
        self.components.analysis.clear_data();// 假设 Analysis 实现了 draw 方法
        self.components.tracing_log_component.draw(frame,layout[2]).expect("绘制图形失败");
        self.components.app_title.draw(frame,layout[0]).expect("绘制图形失败");
        self.components.file_list_component.draw(frame,sub_layout[0]).expect("绘制图形失败");
        self.components.music_progress.draw(frame,playzone_layout[2]).expect("绘制图形失败");
        self.components.lyric_zone.draw(frame,playzone_layout[1]).expect("绘制图形失败");
    }
    //更新组件，主要是更新日志
    pub  fn update_component(&mut self){

        self.components.tracing_log_component.set_log(self.log.join("\n"));

        self.components.lyric_zone.set_lyric(self.sounds_list.get_playingsound_lyric());
    }
    //获取filelist 组件的被选中文件id,并set 到soundlist 里面，用来音频播放
    pub fn set_filelist_component_seleted_item(&mut self) {
        self.sounds_list.playing_item_id=self.components.file_list_component.get_selected_item_id();
    //    println!("get_selected_item_id is {:?}",self.sounds_list.playing_item_id);
    }
    //设置musicprogress 组件的总时长，用于进度条计算
    pub fn set_musicprogress_component_total_duration(&mut self) {

        self.components.music_progress.get_duration(self.sounds_list.get_playingsound_total_durtation().as_secs());

    }

    //获取文件列表
    pub fn get_soundlist(&self)->Vec<(usize,String)>{
        self.sounds_list.get_sound_name_list()
    }
    //设置退出弹框的状态切换
    pub fn quit_toast(&mut self){
      if self.is_quiting==true {
          self.is_quiting=false
      }  else { self.is_quiting=true }
    }
}
pub async fn runner() ->Result<(),MyError>{
    let mut app=App::new();
    let app_clone=Arc::new(Mutex::new(app));
    //println!("sounds is {:?}", app_clone.lock().await.sounds_list.sounds);
    //初始化tracing日志
    //创建发送器和接收器
    let  (mut log,log_receiver) =TracingLog::new();
    log.init_log()?;//初始化日志
    let recv_handle=recv_log(log_receiver,Arc::clone(&app_clone));//把app的log传给recvlog，创建异步任务来接收日志



    // 创建 ActionReactor 和 music 之间的通道
    let (action_sender, mut action_receiver) = std::sync::mpsc::channel();


    let mut tui = Tui::new()?;
    let mut handler=event::EventHandler::new();
    let mut reactor=ActionReactor::new(action_sender.clone(),Arc::clone(&app_clone));
    app_clone.lock().await.init_component(action_sender.clone());

    let mut render=Render::new(tui);
    handler.run();
    let react=tokio::spawn(
            async move {
                loop {
                    if let Some(event_recv) = handler.next().await {
                        // println!("event_recv is {event_recv:?}");
                        reactor.app.lock().await.update_component();
                        reactor.run(event_recv).await;
                    }
                }

        });
    let render1= tokio::spawn(
        {
            let app_clone_render = Arc::clone(&app_clone);
            async move{
                loop {
                    render.run(app_clone_render.clone()).await.expect("渲染失败");
                }

            }
        }
      );

    //设置音乐播放
    let (music_tx, music_reciver) = mpsc::unbounded_channel();//发送和接收fft处理后的数据
    let (sample_sender,sample_receiver)= std::sync::mpsc::channel();;//播放时，发送样本数据给fft


    //fft处理
    let mut fft_controller=FFTController::new(44100.0,4096,sample_receiver,music_tx);
    let fft_result_set_handle=get_fft_result(music_reciver,Arc::clone(&app_clone));
    // let app_clone_fft = Arc::clone(&app_clone);
    // let fft_result_clone=&app_clone_fft.lock().await.fft_result;


    thread::spawn(move || {
        fft_controller.start_process();
    });

    thread::spawn(move||{
        let mut musicplayer = MusicPlayer::new( sample_sender);
        loop {


            if let (action,path) = action_receiver.recv().unwrap() {
                match path{
                    None => {}
                    Some(path) => {musicplayer.set_path(path.clone()); }
                }
                musicplayer.handle_action(action);
            }
        }

    });


    //join handle,等待异步handle 执行完任务，才退出主流程，不然主流程会执行完就退出了
    // spawn 生成的异步task，由Tokio 的任务调度器负责调度任务队列
  tokio::join!(
        //   handler.run(),没有所有权了
           react,
           render1,
           recv_handle,//异步获取tracing 日志
           fft_result_set_handle,
      );


    //检查各个任务的返回结果
    //     if let Err(hanler_err) = Result::<(), _>::Err(hanler_err) {
    //         eprintln!("Error in app run: {:?}", hanler_err);
    //     }
    //     if let Err(reactor_err) = Result::<(), _>::Err(reactor_err) {
    //         eprintln!("Error in app run: {:?}", reactor_err);
    //     }
    //     if let Err(render_err) = Result::<(), _>::Err(render_err) {
    //         eprintln!("Error in app run: {:?}", render_err);
    //     }

    Ok(())
}
