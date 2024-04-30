
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Margin, Rect};
use ratatui::prelude::{Alignment, Stylize};
use ratatui::text::Line;
use ratatui::widgets::{Block, Borders, Padding, Paragraph, Wrap};
use crate::action::Action;
use crate::components::banner::BANNER;
use crate::components::Component;
use crate::error::MyError;

#[derive(Clone)]
pub struct AppTitle{
    pub title: String,
}
impl AppTitle{
    pub fn new()->Self{
        let title=String::from("Ratatui 播放器");
        Self{title}
    }
}
impl Component for AppTitle{
    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<(), MyError> {
        let layout=Layout::new(
            Direction::Horizontal,
            [Constraint::Min(10), Constraint::Length(50)],).split(rect);
        let block=Block::new().borders(Borders::ALL).light_red().padding(Padding::new(0,0,0,0)).title("out");
        let iner_block = Block::default().title("right").borders(Borders::ALL).padding(Padding::zero());

        let paragraph=Paragraph::new(BANNER)
            .block(Block::new().borders(Borders::ALL).title("left").padding(Padding::zero())).yellow()
            .alignment(Alignment::Left).wrap(Wrap { trim:true });
        //f.render_widget(block,rect);
        f.render_widget(paragraph,layout[0].inner(&Margin::new(0,0)));
        let paragraph2=Paragraph::new("q：退出  ↑:上选择  ↓：下选择").block(iner_block)
            .light_yellow().alignment(Alignment::Right).wrap(Wrap { trim:true });
        f.render_widget(paragraph2,layout[1]);
        Ok(())
    }

    fn update(&mut self, action: Option<Action>) -> Result<(), MyError> {
        todo!()
    }

}
