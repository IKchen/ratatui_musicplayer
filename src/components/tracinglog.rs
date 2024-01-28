use std::ptr::addr_of_mut;
use std::sync::{Arc};
use ratatui::{prelude::*, widgets::*};
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use crate::error::MyError;
use super::Component;

use tracing::{Subscriber, Event, event, Level, info, warn};
use crate::app::App;

use crate::tracing::TracingLog;

pub struct TracingLogComponent{
    pub logs: String,//没用过
}

impl TracingLogComponent {
    pub fn new(log:String)->Self{
        let logs =log;
        Self{logs}
    }
}
impl Component for TracingLogComponent{
     fn draw(&mut self, f:&mut ratatui::Frame<'_>, rect: Rect, app:Arc<App>) ->Result<(),MyError>{
        // 这里我们使用克隆的Arc来访问日志
      //  let app_clone =Arc::clone(&app);
        let text = self.logs.clone();
       // println!("text is {:?}",text);
        let layout=Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(100)],
        )
            .split(rect);
        info!("进入draw函数");
         // let rt = Runtime::new().unwrap();
         // let mut text="".to_string();
         // // 在同步函数中执行异步函数
         // rt.block_on(async {
         //     // 你的异步代码
         //    text=(*text_log.lock().await).clone();
         //
         // });
        f.render_widget(Paragraph::new(text)
                            .block( Block::new()
                                .title("tracing日志").red()
                                .borders(Borders::ALL))
                            .blue()
                            .scroll((5, 0)),

                        layout[0]);


        Ok(())
    }
    fn update(& mut self)->Result<(),MyError>{
        Ok(())
    }

    //获取日志 struct 本身，方便其他组件draw 时获取
    // fn get_logging( & self)->TracingLog{
    //     let tracinglog =self.clone();
    //     tracinglog
    //     //self.clone()
    // }

}