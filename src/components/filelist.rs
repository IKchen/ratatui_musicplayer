use std::path::Path;
use ratatui::Frame;
use ratatui::layout::Rect;
use ratatui::{prelude::*, widgets::*};
use tokio::fs;
use tokio::sync::mpsc::UnboundedSender;
use crate::action::Action;
use crate::components::Component;
use crate::error::MyError;
use crate::file::{FileItem, FileList};

#[derive(Clone,Debug)]
pub struct FileListComponent {
    pub filelist:FileList,
    pub vertical_scroll:usize,
    pub vertical_scroll_state:ScrollbarState,
    pub action_tx: Option<UnboundedSender<Action>>,
}

impl  FileListComponent  {
    pub fn new(itemlist:FileList)->Self{

        let mut vertical_scroll=0;
        let mut vertical_scroll_state=ratatui::widgets::ScrollbarState::new(20);
        let mut filelist=itemlist;
        let mut action_tx=None;
        Self{filelist,vertical_scroll,vertical_scroll_state,action_tx}
    }
    pub fn get_item_name(&mut self,index:usize)->String{
        if let Some(item)=self.filelist.item_list.get(index){
            let item_name=item.item_name.clone();
            return item_name
        } else { "noting".to_string() }
    }
    pub fn next(&mut self) {
        let i = match self.filelist.item_status.selected() {
            Some(i) => {
                if i >= self.filelist.item_list.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => self.filelist.last_selected.unwrap_or(0),
        };
        self.filelist.item_status.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.filelist.item_status.selected() {
            Some(i) => {
                if i == 0 {
                    self.filelist.item_list.len() - 1
                } else {
                    i - 1
                }
            }
            None => self.filelist.last_selected.unwrap_or(0),
        };
        self.filelist.item_status.select(Some(i));
    }
}
impl Component for  FileListComponent {
    fn draw(&mut self, f: &mut Frame<'_>, rect: Rect) -> Result<(), MyError> {
        let mut list_1:Vec<ListItem>=Vec::new();
        let mut itemlist=self.filelist.item_list.clone();
        for fileitem in itemlist{
            let mut listitem=ListItem::new(fileitem.item_name).bg(Color::Black);
            list_1.push(listitem);
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
        f.render_stateful_widget(list,rect,&mut self.filelist.item_status);
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
        self.action_tx.clone().unwrap().send(Action::Render)?;
        Ok(())
    }
    fn init(&mut self) -> Result<(), MyError> {
        Ok(())
    }
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) {
        self.action_tx = Some(tx);

    }
}