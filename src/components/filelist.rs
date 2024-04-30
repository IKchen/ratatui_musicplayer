use std::path::Path;
use std::sync::{Arc, Mutex};
use std::sync::mpsc::Sender;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::{prelude::*, widgets::*};
use tokio::fs;
use crate::action::Action;
use crate::app::App;
use crate::components::Component;
use crate::error::MyError;

use crate::sounds::SoundsList;

#[derive(Clone,Debug)]
pub struct FileListComponent {
    pub sound_list:SoundsList,
    pub vertical_scroll:usize,
    pub vertical_scroll_state:ScrollbarState,
    pub action_tx: Option<Sender<Action>>,
}

impl  FileListComponent  {
    pub fn new()->Self{

        let mut vertical_scroll=0;
        let mut vertical_scroll_state=ratatui::widgets::ScrollbarState::new(20);

        let mut action_tx=None;
        Self{sound_list:SoundsList::default(),vertical_scroll,vertical_scroll_state,action_tx}
    }
    pub fn set_file_list(&mut self,sound_list:SoundsList){
        self.sound_list=sound_list;
        self.sound_list.item_status.select(Some(0));//初始设置选中第一条
    }
    pub fn get_selected_item_id(&mut self)->Option<usize>{
        if let Some(item)=self.sound_list.item_status.selected(){

            return Some(item)
        } else { None }

    }
    pub fn next(&mut self) {
        let i = match self.sound_list.item_status.selected() {
            Some(i) => {
                if i >= self.sound_list.sounds.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.sound_list.last_selected.unwrap_or(0),
        };
        self.sound_list.item_status.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.sound_list.item_status.selected() {
            Some(i) => {
                if i == 0 {
                    self.sound_list.sounds.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.sound_list.last_selected.unwrap_or(0),
        };
        self.sound_list.item_status.select(Some(i));
    }
}
impl Component for  FileListComponent {
    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<(), MyError> {
        let mut list_1:Vec<ListItem>=Vec::new();
        let mut itemlist=self.sound_list.sounds.clone();
        for fileitem in itemlist{
            if fileitem.sound_path!=""{
                let mut listitem=ListItem::new(fileitem.name).bg(Color::Black);
                list_1.push(listitem);
            }
        }

        //self.filelist.item_status.select(Some(0));
        let list = List::new(list_1)
            .block( Block::new()
            .title("文件列表").red()
            .borders(Borders::ALL))
            .highlight_style(
                Style::default()
                    .add_modifier(Modifier::BOLD)
                    .add_modifier(Modifier::REVERSED)
                    .fg(Color::Blue),
            )
            .highlight_symbol(">")
            .highlight_spacing(HighlightSpacing::Always);
        f.render_stateful_widget(list,rect,&mut self.sound_list.item_status);
        Ok(())
    }
    fn update(&mut self, action: Option<Action>) -> Result<(), MyError> {
        match action {
            Some(Action::Quit)=> return Ok(()),
            Some(Action::Down) => {
                self.next()
            }
            Some(Action::Up) => {
                self.previous()
            }
            Some(Action::EnterProcessing) => {

            }
            _ => {}
        }
   //     self.action_tx.clone().unwrap().send(Action::Render).unwrap();
        Ok(())
    }
    fn init(&mut self) -> Result<(), MyError> {
        Ok(())
    }
    // fn register_action_handler(&mut self, tx: Sender<Action>) {
    //     self.action_tx = Some(tx);
    //
    // }
}