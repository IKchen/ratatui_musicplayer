use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};
//rodio 是个同步库，只能在同步环境中使用
pub struct MusicPlayer
{
    pub sink: Sink,
    pub stream:rodio::OutputStream,// steam 不能drop 了，不然 handle 就没用了
    pub stream_handle:OutputStreamHandle,
    pub file_path:String,
    pub sender: Sender<f32>,//发送样本数据

}

impl  MusicPlayer {
    pub fn new(sender:Sender<f32>)->Self{
        let (stream, stream_handle) = OutputStream::try_default().unwrap();

        let  sink = Sink::try_new(&stream_handle).unwrap();
        let  file_path=String::from("music/music1.mp3");

        Self{sink,stream,stream_handle,file_path,sender}
    }
    pub  fn handle_action(&mut self, action: Action){
       // info!("action is {action:?}");
        match action {
            Action::Start=>{
                self.play();
                info!("音频开始播放");
            }
            Action::Replay=>{
                self.replay();
                info!("音频已恢复");
            }
            Action::Pause=>{
            //    println!("music is paused {:?}",self.sink.is_paused());
                self.pause();
                info!("音频已暂停");
            }
            Action::Stop=>{
                self.stop();
                info!("append已清空");
           //     println!("music is stop ");
            }
            _=>{
            }
        }
    }
  pub  fn play(&mut self) {

      let sample_sender = self.sender.clone();
      //解码文件
      let file1 = File::open(self.file_path.clone()).expect("文件路径有问题");
      let buf_reader1 = BufReader::new(file1);
      let mut source1 = Decoder::new(buf_reader1).unwrap().convert_samples::<f32>();
      let mut my_source = MyCustomSource::new(source1, sample_sender);
      self.sink.append(my_source);
   //   self.sink.sleep_until_end();//之前外面的线程有loop循环，这里就要不睡眠线程，不然外面recv无法接收到action

  }
    //暂停音乐
    pub fn pause(&self) {
        self.sink.pause();
    }
    // 停止音乐
    pub fn stop(&self) {
        self.sink.stop();
    }
    // 恢复音乐
    pub fn replay(&self) {
        self.sink.play();
    }
    pub fn set_path(&mut self, file:String){
        self.file_path=file;
    }
}


use rodio::{ Sample};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread::spawn;
use std::time::{Duration, Instant};
use tracing::info;

use crate::action::Action;
use crate::app::App;
use crate::error::MyError;

//自定义source类型
pub struct MyCustomSource<I:Source>
where <I as Iterator>::Item: rodio::Sample
{
    input_source: I,
    sample_sender: Sender<f32>,
}
impl <I: rodio::Source> MyCustomSource<I> where <I as Iterator>::Item: rodio::Sample
{
    pub fn new(source:I,sample_sender: Sender<f32>)->Self{
        Self{
            input_source:source,
            sample_sender,
        }
    }
}
impl<I> Iterator for MyCustomSource<I>
    where
        I: Iterator<Item =f32> + rodio::Source,
{
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        self.input_source.next().map(|sample| {
           // println!("sample is {}",sample);
            if let Err(e) = self.sample_sender.send(sample) {//source播放时，发送sample给接收器
                eprintln!("Error sending sample: {}", e);
            }
            sample
        })
    }
}
impl<I> Source for MyCustomSource<I>
    where
        I: Iterator<Item = f32> + Source + 'static,
{
    fn current_frame_len(&self) -> Option<usize> {
        self.input_source.current_frame_len()
    }

    fn channels(&self) -> u16 {
        self.input_source.channels()
    }

    fn sample_rate(&self) -> u32 {
        self.input_source.sample_rate()
    }

    fn total_duration(&self) -> Option<Duration> {
        self.input_source.total_duration()
    }
}
#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use std::thread;
    use tokio::sync::mpsc;
    use crate::action::Action;
    use crate::musicplayer::MusicPlayer;

    #[test]
    fn test_file_open() {
        let file_path = "music/music1.mp3";
        let file = File::open(file_path);

        match file {
            Ok(_) => {
                // 文件打开成功
                assert!(true);
            }
            Err(err) => {
                // 文件打开失败
                println!("Error opening file: {}", err);
                assert!(false);
            }
        }
    }
    #[test]
    fn test_music_pause() {
        // 创建 ActionReactor 和 music 之间的通道
        let (action_sender, mut action_receiver) = std::sync::mpsc::channel();
        let (sample_sender,sample_receiver)= std::sync::mpsc::channel();;//播放时，发送样本数据给fft
        action_sender.send((Action::Pause,"None".to_string())).unwrap();
        thread::spawn(move||{

            let mut musicplayer = MusicPlayer::new( sample_sender);
            loop {
                if let (action,_path) = action_receiver.recv().unwrap() {
                    musicplayer.handle_action(action);
                    assert!(musicplayer.sink.is_paused());
                }
            }

        });
    }
}