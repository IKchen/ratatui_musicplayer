use ratatui::{prelude::*, widgets::*};
use ratatui::prelude::Direction::Vertical;
use crate::error::MyError;
use super::Component;
use std::sync::{Arc, Mutex};
use tracing::{Subscriber, Event, event, Level, info, warn};
use tracing_subscriber::layer::Context;
use tracing_subscriber::Layer;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use log::log;

pub struct TracingLog{
    logs: Arc<Mutex<Vec<String>>>,
}

impl TracingLog {
  pub fn new() -> Self {
        Self{
            logs: Arc::new(Mutex::new(Vec::new())),
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
        let logs_shared = self.logs.clone();
        let subscriber = tracing_subscriber::registry().with(TracingLog {
            logs: logs_shared.clone(),
        });

        tracing::subscriber::set_global_default(subscriber).expect("设置订阅者失败");
        info!("这是一条信息日志");
        warn!("这是一条警告日志");
        // 这里我们使用克隆的Arc来访问日志
        let logs = logs_shared.lock().unwrap();
        let text = logs.join( "\n");
        let layout=Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(50), Constraint::Percentage(50)],
        )
            .split(rect);
        f.render_widget(Paragraph::new(text)
                            .block( Block::new()
                                .title("标题2").red()
                                .borders(Borders::ALL)).blue().on_green(), layout[0]);
        f.render_widget(Paragraph::new("内容3")
                            .block( Block::new()
                                .title("标题3").red()
                                .borders(Borders::ALL)), layout[1]);
        Ok(())
    }
    fn update(& mut self)->Result<(),MyError>{
        Ok(())
    }
}