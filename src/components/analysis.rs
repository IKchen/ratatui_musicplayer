use ratatui::layout::Rect;
use ratatui::prelude::Color;
use ratatui::style::Style;
use ratatui::widgets::{BarChart, Block, Borders};
use tokio::sync::mpsc::UnboundedSender;
use crate::action::Action;
use crate::components::Component;
use crate::error::MyError;

pub struct Analysis<'a> {
    data: Vec<(&'a str, u64)>,
}

impl <'a>Analysis<'a> {
    pub fn new()->Self{
        Analysis{
            data: vec![
                ("B1", 9),
                ("B2", 12),
                ("B3", 5),
                ("B4", 8),
                ("B5", 2),
                ("B6", 4),
                ("B7", 5),
                ("B8", 9),
                ("B9", 14),
                ("B10", 15),
                ("B11", 1),
                ("B12", 0),
                ("B13", 4),
                ("B14", 6),
                ("B15", 4),
                ("B16", 6),
                ("B17", 4),
                ("B18", 7),
                ("B19", 13),
                ("B20", 8),
                ("B21", 11),
                ("B22", 9),
                ("B23", 3),
                ("B24", 5),
            ],
        }
    }
}

impl Component for Analysis<'_>{
    fn  draw(& mut self, f:&mut ratatui::Frame<'_>,rect:Rect)->Result<(),MyError>{
        let barchart = BarChart::default()
            .block(Block::default().title("FFT").borders(Borders::ALL))
            .data(&self.data.clone())
            .bar_width(6)
            .bar_style(Style::default().fg(Color::Yellow))
            .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));

        f.render_widget(barchart, rect);
        Ok(())
    }
    fn  update(& mut self,action:Option<Action> )->Result<(),MyError>{
        Ok(())
    }
    fn init(&mut self) -> Result<(),MyError> {
        Ok(())
    }
    //注册 事件接收器
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) {

    }
}