use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc, Mutex};
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink, Source};

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};


pub struct MusicPlayer
{
    pub sink: Sink,
    pub stream:rodio::OutputStream,// steam 不能drop 了，不然 handle 就没用了
    pub stream_handle:OutputStreamHandle,
    pub file_path:String,
    pub sender: UnboundedSender<f32>,
}

impl  MusicPlayer {
    pub fn new(file:String,sender:UnboundedSender<f32>)->Self{
        let (stream, stream_handle) = OutputStream::try_default().unwrap();

        let  sink = Sink::try_new(&stream_handle).unwrap();

        let file_path=file;

        Self{sink,stream,stream_handle,file_path,sender}
    }
    pub async fn handle_action(&self,action: Action){
        match action {

            Start=>{
            self.sink.play()
            }
            Play=>{

                self.replay()

            }
            Pause=>{
                self.pause()
            }
            _=>{
            }
        }
    }
  pub  async fn play(&self) {

      let file_path = self.file_path.clone();
      let sample_sender = self.sender.clone();
     // let mut sink_clone=Arc::clone(&self.sink);

      //解码文件
      let file1 = File::open("music/music1.mp3").expect("文件路径有问题");
      let buf_reader1 = BufReader::new(file1);
      let mut source1 = Decoder::new(buf_reader1).unwrap().convert_samples::<f32>();
      let mut my_source = MyCustomSource::new(source1, sample_sender);
     // println!("source time is {:?}", my_source.total_duration());
      self.sink.append(my_source);
      self.sink.sleep_until_end();
      // spawn(move || {
      //      let  mut sink_guard=sink_clone.lock().unwrap();
      //   //  if let Some(sink) = &mut *sink_guard { // 解引用MutexGuard并匹配Option,获取解引用后的值的引用，不获取所有权
      //
      //         sink_guard.append(my_source); // 正确调用Sink的方法
      //         println!("播放成功");
      //         sink_guard.sleep_until_end(); // 正确调用Sink的方法
      //        drop(sink_guard);
      //    // }
      //
      // }); // 注意这里添加了await来等待spawn_blocking里的任务完成
      // let  mut sink_guard=sink_clone.lock().unwrap();
      // sink_guard.append(my_source); // 正确调用Sink的方法
      //
      // println!("播放成功");
      // sink_guard.sleep_until_end(); // 正确调用Sink的方法

  }
    pub fn get_music_duration(&mut self)->u64{
        let file_path = self.file_path.clone();
        let path = Path::new(&file_path);
        let time = mp3_duration::from_path(&path).expect("获取时间失败");
        let time_value=time.as_secs();
        time_value
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
    pub fn set_play_path(&mut self,file:String){
        self.file_path=file;
    }
}


use rodio::{ Sample};
use std::sync::mpsc::{channel, Sender};
use std::thread::spawn;
use std::time::{Duration, Instant};
use futures::future::ok;
use tokio::task::JoinHandle;
use crate::action::Action;
use crate::error::MyError;

//自定义source类型
pub struct MyCustomSource<I:Source>
where <I as Iterator>::Item: rodio::Sample
{
    input_source: I,
    sample_sender: UnboundedSender<f32>,
}
impl <I: rodio::Source> MyCustomSource<I> where <I as Iterator>::Item: rodio::Sample
{
    pub fn new(source:I,sample_sender: UnboundedSender<f32>)->Self{
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
}