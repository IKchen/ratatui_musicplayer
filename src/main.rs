mod error;
mod event;
mod tui;
mod components;
mod app;
mod action;
mod render;

use crate::{
    components:: {home::Home,Component,tracinglog::TracingLog}

};
use crossterm::terminal::{self,
                          EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::event::*;
use std::io::{self,Error,Write,Stdout};
use std::panic;
use ratatui::backend::CrosstermBackend;
use crossterm::cursor::EnableBlinking;
use tracing::{Subscriber, Event,event as logevent,Level};
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt};
use tracing::{info, warn};
use app::App;
use crate::error::MyError;


//自定义类型别名,避免类型名称过长
pub type CrosstermTerminal<W> = ratatui::Terminal<ratatui::backend::CrosstermBackend<W>>;

#[tokio::main]
async fn main()->Result<(),MyError>{


   // let subscriber = tracing_subscriber::registry().with(logger);
 //   tracing::subscriber::set_global_default(subscriber).expect("设置订阅者失败");
    let mut app=App::new();
    app.run().await?;
    Ok(())

}

