use std::arch::x86_64::_addcarryx_u64;
use std::fs::File;
use std::io::BufReader;
use rustfft::FftPlanner;
use std::iter::Iterator;
use std::sync::Arc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use futures::{SinkExt, TryFutureExt};
use futures::future::ok;
use log::info;
use rodio::{Decoder, Source};
use rustfft::num_complex::Complex;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use crate::action::Action;
use crate::error::MyError;

pub struct FFTController
{
    pub pause: bool,
    pub stop: bool,
    pub file_path:String,
    pub music_tx: UnboundedSender<Vec<f32>>,
    pub complex_samples: Vec<Complex<f32>>,
    pub sample_rate:f32,
    pub fft_size:usize,
    pub sample_receive:UnboundedReceiver<f32>,
    pub action_sender: UnboundedSender<Action>,//发送action 给render，通知他更新组件
}

impl FFTController

{
    pub fn new(file:String,sample_rate: f32, fft_size: usize,sample_receiver: UnboundedReceiver<f32>,music_tx: UnboundedSender<Vec<f32>>,action_sender: UnboundedSender<Action>) -> Self {
        let file_path=file;
        Self {
            pause: false,
            stop: false,
            file_path,
            music_tx,
            complex_samples: Vec::new(), // 初始化为空，或者你可以根据需要预填充
            sample_rate:44100.0,
            fft_size:4096,
            sample_receive:sample_receiver,
            action_sender,
        }
    }
    pub fn set_pause(&mut self) {
        self.pause = true;
    }
    pub fn set_stop(&mut self) {
        self.stop = true;
    }
    pub async fn start_process(&mut self)->JoinHandle<Result<(), MyError>>

    {
       //-----打开音频文件的实现
        // let music_tx=self.music_tx.clone();
        // let file2 = File::open(self.file_path.clone()).expect("文件路径有问题");
        // let mut source2=Decoder::new( BufReader::new(file2)).unwrap();
        // let mut samples=source2.convert_samples::<f32>();//转f32
        // let mut music_sender=self.music_tx.clone();
        // let mut planner = FftPlanner::new();
        // let fft = planner.plan_fft_forward(4096); // 假设使用 4096 点 FFT
        //      for sample in samples {
        //
        //          // 将样本与汉宁窗进行逐元素相乘
        //          let windowed_sample = sample * 1.0;
        //          self.complex_samples.push(Complex::from(windowed_sample));
        //          if self.complex_samples.len() >= 4096{
        //              fft.process(self.complex_samples.as_mut_slice());
        //              self.map_fft_to_notes();
        //              // 清空缓冲区，准备下一轮数据
        //              self.complex_samples.clear();
        //          }
        //      }
        tokio::spawn({
            //-------接收receiver版本的实现
            let mut planner = FftPlanner::new();
            let fft = planner.plan_fft_forward(4096); // 假设使用 4096 点 FFT
            while let sample=self.sample_receive.recv().await.unwrap() {
                self.complex_samples.push(Complex::from(sample));
                if self.complex_samples.len() >= 4096{
                    fft.process(self.complex_samples.as_mut_slice());
                    self.map_fft_to_notes();
                    // 清空缓冲区，准备下一轮数据
                    self.complex_samples.clear();
                }
            }
            ok(())
        })

    }

    //先将频率转换成MIDI音符编号
    fn freq_to_midi_number(&self, freq: f32) -> i32 {
        let a4_freq = 440.0;
        let a4_midi_number = 69;
        (((freq / a4_freq).log2()) * 12.0).round() as i32 + a4_midi_number
    }
    //然后根据MIDI编号获取音符名称
    fn midi_number_to_note_name(&self, midi_number: i32) -> &'static str {
        let notes = ["C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B"];
        let note_index = (midi_number % 12) as usize;
        notes[note_index]
    }

    // 将FFT结果映射到12平均律的音符上
    fn map_fft_to_notes(&mut self) {
        let freq_resolution = self.sample_rate / self.fft_size as f32;
        let mut note_energies = vec![0.0; 12]; // 存储每个音符的能量

        for (i, complex) in self.complex_samples.iter().enumerate() {
            let freq = i as f32 * freq_resolution; // 当前频率分量的频率
            if freq > 20.0 && freq < 4200.0 { // 只关注人耳可听范围内的频率
                let midi_number = self.freq_to_midi_number(freq);
                if midi_number >= 60 && midi_number <= 71 { // C4到B4
                    let energy = complex.norm(); // 这个频率分量的能量
                    let note_index = (midi_number % 12) as usize;
                    note_energies[note_index] += energy; // 累加到相应音符的能量上
                }
            }
        }

        // 此处可以根据note_energies向量将能量发送出去或进行可视化处理
        // 例如打印每个音符的能量
        for (i, &energy) in note_energies.iter().enumerate() {
            let note_name = self.midi_number_to_note_name(i as i32 + 60); // C4开始

          //  println!("{}: {}", note_name, energy,);
        }
     //   self.action_sender.send(Action::Render).unwrap();//发送更新动作，让render组件接收fft结果，然后重新渲染
        self.music_tx.clone().send(note_energies).expect("fft发送失败");
    }
}

 pub async fn get_fft_result(mut music_reciver:UnboundedReceiver<Vec<f32>>, fft_result_buffer:Arc<Mutex<Vec<(String, u64)>>>) ->(JoinHandle<()>){
     *fft_result_buffer.lock().await=vec![
         ("B1".to_string(), 0),
         ("B2".to_string(), 0),
         ("B3".to_string(), 0),
         ("B4".to_string(), 0),
         ("B5".to_string(), 0),
         ("B6".to_string(),0),
         ("B7".to_string(), 0),
         ("B8".to_string(), 0),
         ("B9".to_string(), 0),
         ("B10".to_string(), 0),
         ("B11".to_string(), 0),
         ("B12".to_string(), 0),
     ];
    let get_fft_result_handle= tokio::spawn(async move{
        let mut fft_buffer_clone=Arc::clone(&fft_result_buffer);
         while let mut fft_result=music_reciver.recv().await.unwrap(){
             info!("更新data数据");
             //  println!("fft 是{:?}",fft_result);
          //   let mut buffer = fft_result_buffer.lock().await;
             for ((name, value), new_value) in fft_buffer_clone.lock().await.iter_mut().zip(fft_result.iter()) {
                 *value = *new_value as u64; // 将第一个数组中的值替换为第二个数组的值
             }
            //   println!("data is {:?}",fft_result_buffer);
         }
     });

    get_fft_result_handle
}

