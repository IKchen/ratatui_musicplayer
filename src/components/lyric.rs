use std::sync::mpsc::Sender;
use std::time::Duration;
use ratatui::prelude::{Color, Line, Modifier, Style};
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::prelude::{Alignment, Stylize};
use ratatui::widgets::{Block, Borders, HighlightSpacing, List, ListItem, ListState};
use tokio::sync::mpsc::UnboundedSender;
use tokio::time::Instant;
use crate::action::Action;
use crate::components::Component;
use crate::error::MyError;
use crate::lyric::Lyric;

#[derive(Clone)]
pub struct LyricZone{
    pub lyric:Vec<Lyric>,
    pub lyric_state:ListState,
    pub start:Instant,
    pub action_tx: Option<Sender<Action>>,
}
impl LyricZone{
    pub fn new()->Self{

        Self{lyric:vec![Lyric::new()],lyric_state:ListState::default(),start:Instant::now(),action_tx:None}
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
    pub fn set_lyric(&mut self,lyric:Vec<Lyric>){
        self.lyric=lyric;
        self.lyric_state=ListState::default();//重置歌词选中状态
        self.start=Instant::now();//重置歌词时间
    }
}
impl Component for LyricZone{
    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<(), MyError> {
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
            let seconds = time.1.parse::<u64>().unwrap_or(0);

            // 将分钟和秒数转换为Duration
            let now_duration = Duration::new(minutes * 60 + seconds, 0);
            if self.start.elapsed()>now_duration{//时长大于当前的时间戳，就往下走一句
                self.next()
            }
        }

        Ok(())
    }
    // fn register_action_handler(&mut self, tx: Sender<Action>) {
    //     self.action_tx=Some(tx);
    // }

}