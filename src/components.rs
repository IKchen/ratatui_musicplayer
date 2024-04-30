use std::sync::{Arc};
use std::sync::mpsc::Sender;
use ratatui::layout::Rect;
use tokio::sync::mpsc::UnboundedSender;
use tokio::sync::Mutex;
use crate::action::Action;
use crate::app::App;
use crate::components::analysis::Analysis;
use crate::components::apptitle::AppTitle;
use crate::components::filelist::FileListComponent;
use crate::components::lyric::LyricZone;
use crate::components::musicprogress::MusicProgress;
use crate::components::playzone::PlayZone;
use crate::components::quit::Quit;
use crate::components::tracinglog::TracingLogComponent;
use crate::error::MyError;
use crate::tracing::TracingLog;
//引入组件
pub mod home;

pub mod quit;
pub mod tracinglog;
pub mod filelist;
pub mod playzone;
pub mod analysis;
pub mod musicprogress;
pub mod lyric;
pub mod apptitle;
mod banner;


pub trait Component{

    fn  draw(& mut self, f:&mut ratatui::Frame<'_>,rect:Rect)->Result<(),MyError>;
    fn  update(& mut self,action:Option<Action> )->Result<(),MyError>;
    fn init(&mut self) -> Result<(),MyError> {
        Ok(())
    }
    //注册 事件接收器
    fn register_action_handler(&mut self, tx: Sender<Action>) {

    }

}
#[derive(Clone)]
pub struct Components{
    pub analysis: Analysis,
    pub app_title: AppTitle,
    pub file_list_component: FileListComponent,
    pub lyric_zone: LyricZone,
    pub music_progress: MusicProgress,
    pub play_zone: PlayZone,
    pub tracing_log_component: TracingLogComponent,
    pub quit:Quit
}

impl Components {
    pub fn new()->Self{
        Self{
            play_zone:PlayZone::new(),
            analysis: Analysis::new(),
            music_progress:MusicProgress::new(),
            lyric_zone:LyricZone::new(),
            app_title: AppTitle::new(),
            file_list_component:FileListComponent::new(),
            quit:Quit::new(),
            tracing_log_component: TracingLogComponent::new(),
        }
    }
}
