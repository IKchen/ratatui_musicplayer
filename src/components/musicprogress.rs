use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::*;
use crate::action::Action;
use crate::components::Component;
use crate::error::MyError;

pub struct MusicProgress{
    pub progress: u16,
    pub total_duration:u64,
    pub achive_duration:Duration,
    pub start:Instant
}
impl MusicProgress{
    pub fn new()->Self{
        let progress=0;
        Self{progress,total_duration:0,achive_duration:Duration::from_secs(0), start:Instant::now()}
    }
    pub fn get_duration(&mut self,total_duration:u64){
        self.total_duration=total_duration;
    }
    pub fn start_count(&mut self){
        let achive=Arc::new(Mutex::new(self.achive_duration));
        let start = Instant::now();
      //   let total=self.total_duration.clone();
      // thread::spawn(move || {
      //
      //       loop {
      //           let elapsed = start.elapsed();
      //           if elapsed.as_secs() >= total{
      //               break;
      //           }
      //           // 更新计时器的值
      //           *achive.lock().expect("获取失败") = elapsed;
      //           // 休眠一段时间，以减少 CPU 使用率
      //           thread::sleep(Duration::from_millis(100));
      //       }
      //   });

    }
    pub fn set_count(&mut self){
        if self.start.elapsed().as_secs()>=self.total_duration { self.achive_duration=Duration::from_secs(0) }
        else {  self.achive_duration=self.start.elapsed(); }
    }
}
impl Component for  MusicProgress{
    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<(), MyError> {
        self.progress= ((self.achive_duration.as_secs()as f64 / self.total_duration as f64 ) *100.0) as u16;
        let label = Span::styled(
            format!("{}:{}/{}:{}",self.achive_duration.as_secs()/60,self.achive_duration.as_secs() % 60,self.total_duration/60,self.total_duration % 60),
            Style::new().italic().bold().fg(Color::Cyan),
        );
        let progress=Gauge::default()
            .block(Block::default().borders(Borders::ALL).title("Progress"))
            .gauge_style(
                Style::default()
                    .fg(Color::White)
                    .bg(Color::Black)
                    .add_modifier(Modifier::ITALIC),
            )
            .percent(self.progress)
            .label(label);
        f.render_widget(progress,rect);
        Ok(())
    }

    fn update(&mut self, action: Option<Action>) -> Result<(), MyError> {
        todo!()
    }

}