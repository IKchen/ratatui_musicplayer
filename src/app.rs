use std::string::String;
use std::ops::DerefMut;
use std::sync::{Arc};
use std::sync::mpsc::channel;
use std::thread;
use std::time::Duration;
use futures::future::try_join;
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
use crate::components::Component;
use crate::{app, event};
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
    // pub path:String,

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

        Self{should_quit,frame_rate,tick_rate,sounds_list,fft_result,log}
    }
    //根据目录，初始化文件列表

    //获取文件列表
    pub fn get_soundlist(&self)->Vec<(u32,String)>{
        self.sounds_list.get_sound_name_list()
    }
}
pub async fn runner(mut app:App) ->Result<(),MyError>{

    let app_clone=Arc::new(Mutex::new(app));


    //初始化tracing日志
    //创建发送器和接收器
    let  (mut log,log_receiver) =TracingLog::new();
    log.init_log()?;//初始化日志
    let recv_handle=recv_log(log_receiver,Arc::clone(&app_clone));//把app的log传给recvlog，创建异步任务来接收日志



    // 创建 EventHandler 和 ActionReactor 之间的通道
    let (event_sender, event_receiver) = mpsc::unbounded_channel();

    // 创建 ActionReactor 和 Renderer 之间的通道
    let (action_sender, action_receiver) = mpsc::unbounded_channel();
    let mut tui = Tui::new()?;
    let mut handler=event::EventHandler::new(event_sender);

    //设置音乐播放
    let (music_tx, music_reciver) = mpsc::unbounded_channel();//发送和接收fft处理后的数据
    let (sample_sender,samole_receiver)=mpsc::unbounded_channel();//播放时，发送样本数据给fft
    let mut musicplayer=MusicPlayer::new("music/music1.mp3".to_string(),sample_sender);
    let mut action_sender_clone=action_sender.clone();
    //音乐进度初始化
    let mut total_time=musicplayer.get_music_duration();

    //fft处理
    let mut fft_controller=FFTController::new("music/music1.mp3".to_string(),44100.0,4096,samole_receiver,music_tx,action_sender_clone);
    let mut fft_result_buffer=Arc::new(Mutex::new(Vec::new()));
    let fft_result_set_handle=get_fft_result(music_reciver,Arc::clone(&app_clone));

    let mut lyric=LyricController::new("./music".to_string());
    lyric.get_file().expect("歌词文件获取失败");
    let initaled_lyric=lyric.inital_lyric();

    //把通道接收端，发送端传递给 reactor 和render
    let mut reactor=ActionReactor::new(action_sender.clone(),event_receiver);
    let mut render=Render::new(action_receiver, tui);//把存放的日志传进去

    //join handle,等待异步handle 执行完任务，才退出主流程，不然主流程会执行完就退出了
    // spawn 生成的异步task，由Tokio 的任务调度器负责调度任务队列
    let (hanler_err, reactor_err,
        render_err,_,_,
        _) = tokio::join!(
            handler.run(),
            reactor.run(),
            render.run(Arc::clone(&app_clone),action_sender.clone(),Arc::clone(&fft_result_buffer),total_time,initaled_lyric),
            recv_handle,//异步获取tracing 日志
            fft_controller.start_process(),
            fft_result_set_handle
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
