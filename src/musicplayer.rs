use std::fs::File;
use std::io::BufReader;
use std::thread;
use rodio::{Decoder, OutputStream, Sink, Source};
use rodio::source::Buffered;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::oneshot;
use crate::action::Action;
use crate::fft::FFTController;

pub struct MusicPlayer
{
    //pub source:Decoder<Buffered<File>>,
  //  pub sink:Sink,
    pub file_path:String,
    pub sender: UnboundedSender<f32>,
}

impl  MusicPlayer {
    pub fn new(file:String,sender:UnboundedSender<f32>)->Self{
        // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        // let sink = Sink::try_new(&stream_handle).unwrap();
        let file_path=file;

        Self{file_path,sender}
    }
  pub async fn play(&self) {
      //   let (tx, rx) = oneshot::channel::<()>();
      //   let file_path =self.file_path.clone();
      //   let (_stream, stream_handle) = OutputStream::try_default().unwrap();
      //   let sink = Sink::try_new(&stream_handle).unwrap();
      //   thread::spawn(move||{
      //       let file1 = File::open(file_path.clone()).expect("文件路径有问题");
      //       let buf_reader1 = BufReader::new(file1);
      //       let mut source1 = Decoder::new(buf_reader1).unwrap();
      //
      //       sink.append(source1);
      //       sink.sleep_until_end();
      //       // 发送通知表示播放完成
      //       let _ = tx.send(());
      //   });
      // rx.await?;// 异步等待音频播放完成的通知
      // Ok(())
      let file_path = self.file_path.clone();
      let sample_sender = self.sender.clone();
      tokio::task::spawn_blocking(move || {

          let (_stream, stream_handle) = OutputStream::try_default().unwrap();
          let sink = Sink::try_new(&stream_handle).unwrap();
          let file1 = File::open(file_path.clone()).expect("文件路径有问题");
          let buf_reader1 = BufReader::new(file1);
          let mut source1 = Decoder::new(buf_reader1).unwrap().convert_samples::<f32>();
          let mut my_source = MyCustomSource::new(source1, sample_sender);

          sink.append(my_source);
          sink.sleep_until_end();
      }).await.unwrap(); // 注意这里添加了await来等待spawn_blocking里的任务完成
  }

}

use rodio::{ Sample};
use std::sync::mpsc::{channel, Sender};
use std::time::Duration;
use futures::future::ok;
use tokio::task::JoinHandle;
use crate::error::MyError;


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
