use ratatui::{prelude::*, widgets::*};
use ratatui::prelude::Direction::Vertical;
use crate::error::MyError;
use super::Component;
use std::sync::Mutex;
use tracing::{Subscriber, Event, event,Level};
use tracing_subscriber::layer::Context;
use tracing_subscriber::Layer;
pub struct TracingLog{
    logs: Mutex<Vec<String>>,
}

impl TracingLog {
  pub fn new() -> Self {
        Self{
            logs: Mutex::new(Vec::new()),
        }
    }
}
impl<S: Subscriber> Layer<S> for TracingLog {
    fn on_event(&self, event: &Event<'_>, _ctx: Context<'_, S>) {
        let mut logs = self.logs.lock().unwrap();
        // 将事件格式化为字符串并存储
        let log = format!("{:?}", event);
        logs.push(log);
        // 这里可以添加逻辑以限制日志的大小
    }
}
impl Component for TracingLog{
    fn draw(&mut self ,f:&mut ratatui::Frame<'_>,rect: Rect)->Result<(),MyError>{
        event!(Level::INFO, "KAISHI XUANRAN");
        let layout=Layout::default()
            .direction(Vertical)
            .constraints(vec![Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(Rect::new(0, 0, 10, 10));
        f.render_widget(Paragraph::new("内容1")
                            .block( Block::new()
                                .title("标题1").red()
                                .borders(Borders::ALL)).blue().on_green(), layout[0]);
        f.render_widget(Paragraph::new("内容2")
                            .block( Block::new()
                                .title("标题2").red()
                                .borders(Borders::ALL)), layout[1]);
        Ok(())
    }
    fn update(& mut self)->Result<(),MyError>{
        Ok(())
    }
}