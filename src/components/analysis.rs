use std::sync::{Arc, Mutex};
use std::sync::mpsc::Receiver;
use std::thread;
use log::info;
use ratatui::layout::Rect;
use ratatui::prelude::Color;
use ratatui::style::Style;
use ratatui::text::Line;
use ratatui::widgets::{Bar, BarChart, BarGroup, Block, Borders};
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use crate::action::Action;
use crate::components::Component;
use crate::error::MyError;

pub struct Analysis{
    pub data: Vec<(String, u64)>,

    pub action_sender:UnboundedSender<Action>,
}

impl Analysis{
    pub fn new(
                action:UnboundedSender<Action> )->Self{

        Analysis{
            data: vec![
                ("B1".to_string(), 0),
                ("B2".to_string(), 0),
                ("B3".to_string(), 0),
                ("B4".to_string(),0),
                ("B5".to_string(), 0),
                ("B6".to_string(), 0),
                ("B7".to_string(), 0),
                ("B8".to_string(), 0),
                ("B9".to_string(), 0),
                ("B10".to_string(), 0),
                ("B11".to_string(), 0),
                ("B12".to_string(), 0),
            ],
         //   music_reciver,
            action_sender:action
        }
    }
    // pub async fn get_fft_result(&mut self){
    //
    //         while let mut fft_result=self.music_reciver.recv().await.unwrap(){
    //             info!("更新data数据");
    //          //  println!("fft 是{:?}",fft_result);
    //             for ((name, value), new_value) in self.data.iter_mut().zip(fft_result.iter()) {
    //                 *value = *new_value as u64; // 将第一个数组中的值替换为第二个数组的值
    //             }
    //           //  println!("data is {:?}",self.data);
    //         }
    //
    //
    // }
    pub  fn set_data(&mut self,data:Vec<(String, u64)>){
        self.data= data
    }
    pub fn clear_data(&mut self){
        self.data.clear()
    }
}

impl Component for Analysis{
    fn  draw(& mut self, f:&mut ratatui::Frame<'_>,rect:Rect)->Result<(),MyError>{
        let mut newdata: Vec<_>=self.data.iter().map(|(barname,value)|{
            let groupdata=Bar::default().label(Line::from(barname.clone())).value(*value);
            groupdata
        }).collect();
        let mut group=BarGroup::default().bars(&newdata);

        let barchart = BarChart::default()
            .block(Block::default().title("FFT").borders(Borders::ALL))
            .data(group)
            .bar_width(6)
            .bar_style(Style::default().fg(Color::Yellow))
            .value_style(Style::default().fg(Color::Black).bg(Color::Yellow));

        f.render_widget(barchart, rect);

        Ok(())
    }
    fn  update(& mut self,action:Option<Action> )->Result<(),MyError>{
        match action {
            Some(Action::Update) => {

            },
            _ => {}
        }
        self.action_sender.send(Action::Render).expect("组件发送动作失败");
        Ok(())
    }
    fn init(&mut self) -> Result<(),MyError> {
        Ok(())
    }
    //注册 事件接收器
    fn register_action_handler(&mut self, tx: UnboundedSender<Action>) {

    }
}