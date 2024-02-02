mod error;
mod event;
mod tui;
mod components;
mod app;
mod action;
mod render;
mod tracing;
mod file;
//mod config;

use crate::components::Component;
use std::io::Write;
// use tracing::{Event, event as logevent, Level, Subscriber};
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// use tracing::{info, warn};
use app::App;
use crate::app::runner;
use crate::error::MyError;


//自定义类型别名,避免类型名称过长
pub type CrosstermTerminal<W> = ratatui::Terminal<ratatui::backend::CrosstermBackend<W>>;

#[tokio::main]
async fn main()->Result<(),MyError>{


   // let subscriber = tracing_subscriber::registry().with(logger);
 //   tracing::subscriber::set_global_default(subscriber).expect("设置订阅者失败");
    let mut app=App::new();
   // app.run().await?;
    runner(app).await?;
 //    let mut tick_interval = tokio::time::interval(Duration::from_secs_f64(1.0 / 4.0));
 //    let mut render_interval = tokio::time::interval(Duration::from_secs_f64(1.0 / 60.0));
 //    let mut cancelation_token = CancellationToken::new();
 //    let start_time = Instant::now();
 //    loop {
 //        let current_time = Instant::now();
 //        //  let crossterm_event =  futures::StreamExt::next(&mut reader).fuse();
 //        if cancelation_token.is_cancelled() {
 //            println!("is_cancelled is cancelled");
 //            break;
 //        }
 //        let elapsed_time = current_time - start_time;
 //        if elapsed_time >= Duration::from_secs(5) {
 //            println!("is_cancelled is 准备取消1" );
 //            cancelation_token.cancel();
 //
 //        }
 //        tokio::select! {
 //
 //                    _ = tick_interval.tick()=> {
 //                                println!("tick_interval is {:?}", tick_interval);
 //
 //                    }
 //                    _ = render_interval.tick()=> {
 //                        // 处理渲染逻辑
 //                                println!("render_interval is {:?}", render_interval);
 //                    }
 //                }
 //
 //    }

    Ok(())
}

