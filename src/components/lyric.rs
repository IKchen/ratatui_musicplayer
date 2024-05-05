use std::sync::mpsc::Sender;
use std::time::{Duration, Instant};
use ratatui::prelude::{Color, Line, Modifier, Style};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Alignment, Stylize};
use ratatui::widgets::{Block, Borders, HighlightSpacing, List, ListItem, ListState};
use tokio::sync::mpsc::UnboundedSender;
use tracing::info;
use crate::action::Action;
use crate::components::Component;
use crate::error::MyError;
use crate::lyric::Lyric;

#[derive(Clone)]
pub struct LyricZone{
    pub lyric:Vec<Lyric>,
    pub lyric_state:ListState,
    pub start:Instant,
    pub elapsed: Duration,//累计计时
    pub action_tx: Option<Sender<Action>>,
    pub running: bool,
}
impl LyricZone{
    pub fn new()->Self{

        Self{lyric:vec![Lyric::new()],lyric_state:ListState::default(),start:Instant::now(),elapsed: Duration::new(0, 0),action_tx:None,running:false}
    }
    pub fn next(&mut self) {
        let i = match self.lyric_state.selected() {
            Some(i) => {
                if i >= self.lyric.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.lyric_state.select(Some(i));
        if i>5 { *self.lyric_state.offset_mut()=i-5; }

    }
    //初始化设置
    pub fn set_lyric(&mut self,lyric:Vec<Lyric>){
        self.lyric=lyric;
        self.lyric_state=ListState::default();//重置歌词选中状态
        self.reset();
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
      //  println!("音频时间开始,action is {action:?}");

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
impl Component for LyricZone{
    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<(), MyError> {
        self.elapsed();//每次渲染更新一下计时
        let lyric_item:Vec<ListItem>=self.lyric.iter().map(|s|{
            let l=Line::from(s.lyric.clone()).alignment(Alignment::Center);
            let l_item=ListItem::new(l);
            l_item
        }).collect();
        let lyric=List::new(lyric_item)
            .block( Block::new()
                .title("歌词").red()
                .borders(Borders::ALL))
           // .blue()
            .highlight_spacing(HighlightSpacing::WhenSelected)
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Green),
                   // .fg(Color::LightYellow),
            )
            .highlight_symbol("Sing:");
        f.render_stateful_widget(lyric,rect,&mut self.lyric_state);
        Ok(())
    }

    fn update(&mut self, action: Option<Action>) -> Result<(), MyError> {
        let i = self.lyric_state.selected().unwrap_or(0);
        if let Some(time) = self.lyric[i].time.split_once(':') {
            let minutes = time.0.parse::<u64>().unwrap_or(0);
            // 分割秒和小数部分

            let seconds_and_decimals: Vec<&str> = time.1.split('.').collect();
            let seconds: u64 = seconds_and_decimals[0].parse().unwrap_or(0);
            let decimals: u64 = seconds_and_decimals.get(1).map_or(0, |x| x.parse().unwrap_or(0));


            // 将分钟和秒数转换为Duration
            let now_duration = Duration::new(minutes * 60 + seconds, 0) + Duration::from_millis(decimals * 10);
            if self.elapsed>=now_duration{//时长大于当前的时间戳，就往下走一句
                self.next()
            }
        }


        Ok(())
    }
    // fn register_action_handler(&mut self, tx: Sender<Action>) {
    //     self.action_tx=Some(tx);
    // }

}
#[cfg(test)]
mod tests {
    use std::time::Duration;


    //测试通道是否畅通
    #[tokio::test]
    async fn test_time_transform() {
        let string="00:45.59".to_string();
        if let Some(time) = string.split_once(':') {
            let minutes = time.0.parse::<u64>().unwrap_or(0);
            let seconds_with_decimals = time.1;
            // 分割秒和小数部分
            // 解析分钟和秒

            let seconds_and_decimals: Vec<&str> = time.1.split('.').collect();
            let seconds: u64 = seconds_and_decimals[0].parse().unwrap_or(0);
            let decimals: u64 = seconds_and_decimals.get(1).map_or(0, |x| x.parse().unwrap_or(0));



            assert_eq!(minutes,0);
            assert_eq!(seconds,45);
            assert_eq!(decimals,59);

            // 将分钟和秒数转换为Duration
            let now_duration = Duration::new(minutes * 60 + seconds, 0) + Duration::from_millis(decimals * 10);
        }

    }
}