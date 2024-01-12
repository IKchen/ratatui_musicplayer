use std::sync::Arc;
use ratatui::{prelude::*, widgets::*};
use crate::error::MyError;
use super::Component;

use tracing::{Subscriber, Event, event, Level, info, warn};

use crate::tracing::TracingLog;

impl Component for TracingLog{
    fn draw(&mut self ,f:&mut ratatui::Frame<'_>,rect: Rect)->Result<(),MyError>{
        // 这里我们使用克隆的Arc来访问日志
        let logs = self.get_log();//获取logo值
        let text = logs.lock().unwrap().join( "\n");
       // println!("text is {:?}",text);
        let layout=Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(100)],
        )
            .split(rect);
        info!("进入draw函数");
        f.render_widget(Paragraph::new(text)
                            .block( Block::new()
                                .title("tracing日志").red()
                                .borders(Borders::ALL)).blue(), layout[0]);

        Ok(())
    }
    fn update(& mut self)->Result<(),MyError>{
        Ok(())
    }

    //获取日志 struct 本身，方便其他组件draw 时获取
    fn get_logging( & self)->TracingLog{
        let tracinglog =self.clone();
        tracinglog
        //self.clone()
    }

}