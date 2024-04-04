use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::sync::{Arc, Mutex};
use rodio::{Decoder, OutputStream, Sink, Source};

use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};


pub struct MusicPlayer
{
    //pub source:Decoder<Buffered<File>>,
    pub sink: Arc<Mutex<Option<Sink>>>,
    pub file_path:String,
    pub sender: UnboundedSender<f32>,
}

impl  MusicPlayer {
    pub fn new(file:String,sender:UnboundedSender<f32>)->Self{
        // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink =Arc::new(Mutex::new(Some(Sink::try_new(&stream_handle).unwrap()))) ;
     //   let sink = Arc::new(Mutex::new(None)); // 初始时没有播放，因此为None
        let file_path=file;

        Self{sink,file_path,sender}
    }
  pub  fn play(&self) {

      let file_path = self.file_path.clone();
      let sample_sender = self.sender.clone();
      let sink_clone=Arc::clone(&self.sink);

      //解码文件
      let file1 = File::open(file_path.clone()).expect("文件路径有问题");
      let buf_reader1 = BufReader::new(file1);
      let mut source1 = Decoder::new(buf_reader1).unwrap().convert_samples::<f32>();
      let mut my_source = MyCustomSource::new(source1, sample_sender);


     // let start = Instant::now(); // 开始计时
      spawn(move || {
           let  mut sink_guard=sink_clone.lock().unwrap();
          if let Some(sink) = &mut *sink_guard { // 解引用MutexGuard并匹配Option,获取解引用后的值的引用，不获取所有权
             println!("播放成功");
              sink.append(my_source); // 正确调用Sink的方法
              sink.sleep_until_end(); // 正确调用Sink的方法

          }
          // *sink.append(my_source);
          // *sink_clone.lock().unwrap().unwrap().append(my_source);


          //*sink_clone.lock().unwrap().sleep_until_end();


      }); // 注意这里添加了await来等待spawn_blocking里的任务完成
     // *achive_duration.lock().await= start.elapsed(); // 计算经过的时间
      // println!("achive_duration is {:?},total_duration is {:?}",*achive_duration.lock().await,*total_duration.lock().await);
    //  if  *achive_duration.lock().await>=*total_duration.lock().await{ *achive_duration.lock().await=Duration::from_secs(0) }




  }
    pub fn get_music_duration(&mut self)->u64{
        let file_path = self.file_path.clone();
        let path = Path::new(&file_path);
        let time = mp3_duration::from_path(&path).expect("获取时间失败");
        let time_value=time.as_secs();
        time_value
    }
    // 暂停音乐
    pub fn pause(&self) {
        let sink_guard = self.sink.lock().unwrap();
        if let Some(ref sink) = *sink_guard {
            sink.pause();
        }
    }
    // 停止音乐
    pub fn stop(&self) {
        let mut sink_guard = self.sink.lock().unwrap();
        *sink_guard = None; // 通过设置为None来停止播放，播放线程会等待不会结束
       // *sink_guard.stop() // 停止播放，并清空序列，播放线程会结束
    }
    // 恢复音乐
    pub fn replay(&self) {
        let mut sink_guard = self.sink.lock().unwrap();
        if let Some(ref sink) = *sink_guard {
            sink.play();
        }
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
