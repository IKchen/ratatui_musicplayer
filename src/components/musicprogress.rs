use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::*;
use ratatui::widgets::*;
use tracing::info;
use crate::action::Action;
use crate::components::Component;
use crate::error::MyError;

#[derive(Clone)]
pub struct MusicProgress{
    pub progress: u16,
    pub total_duration:u64,//歌词总时长
    pub start:Instant,
    pub elapsed: Duration,//累计计时
    pub running: bool,
}
impl MusicProgress{
    pub fn new()->Self{
        let progress=0;
        Self{progress,total_duration:0, start:Instant::now(),elapsed: Duration::new(0, 0),running:false}
    }
    pub fn get_duration(&mut self,total_duration:u64){
        self.total_duration=total_duration;
        self.reset();//获得总时长后重置计时
    }

    // 停止秒表
    pub fn stop(&mut self) {
            self.elapsed += Instant::now().duration_since(self.start);
    }
    // 获取当前秒表时间
    pub fn elapsed(&mut self) -> Duration {

        if self.running {
            self.elapsed = self.elapsed + Instant::now().duration_since(self.start);
            self.start = Instant::now(); // 重置开始时间，以防下次调用
        }
        self.elapsed
    }
    // 启动秒表
    pub fn start_count(&mut self) {
        self.start = Instant::now();

    }
    // 重置秒表
    pub fn reset(&mut self) {
        self.elapsed = Duration::new(0, 0);//重置歌词计时时间
        self.start = Instant::now(); // 可选：重置开始时间，如果立即重启
    }
    pub fn handle_action(&mut self, action: Option<Action>) -> Result<(), MyError> {

        match action {
            Some(Action::Pause) => {
                self.stop();
                self.elapsed();
                self.running=false;

                info!("时间是{:?}",self.elapsed)
            }
            Some(Action::Replay) => {
                self.start_count();
                self.elapsed();
                self.running=true;
            }
            Some(Action::Start)=>{
                self.running=true;
                self.reset();


            }
            _=>{}
        }
        Ok(())
    }
}
impl Component for  MusicProgress{
    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<(), MyError> {
        self.elapsed();//每次渲染更新一下计时
        self.progress= ((self.elapsed.as_secs()as f64 / self.total_duration as f64 ) *100.0) as u16;
        let label = Span::styled(
            format!("{}:{}/{}:{}",self.elapsed.as_secs()/60,self.elapsed.as_secs() % 60,self.total_duration/60,self.total_duration % 60),
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
        println!("音频时间开始,action is {action:?}");
        match action {
            Some(Action::Pause) => {
                self.running=false;
                self.elapsed();
            }
            Some(Action::Replay) => {
                self.running=true;
                self.elapsed();
            }
            Some(Action::Start)=>{
                self.running=true;
                self.reset();
                self.start_count();
                info!("音频时间开始")
            }
            _=>{}
        }
        Ok(())
    }

}