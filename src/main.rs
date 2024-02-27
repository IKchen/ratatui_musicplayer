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

use std::f32::consts::PI;
use crate::components::Component;
use std::io::Write;
// use tracing::{Event, event as logevent, Level, Subscriber};
// use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// use tracing::{info, warn};
use app::App;
use crate::app::runner;
use crate::error::MyError;
use rodio::{Decoder, OutputStream, Sink, Source};
use rustfft::num_complex::Complex;
use rustfft::FftPlanner;
use std::fs::File;
use std::io::BufReader;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use rustfft::num_traits::ToPrimitive;

//自定义类型别名,避免类型名称过长
pub type CrosstermTerminal<W> = ratatui::Terminal<ratatui::backend::CrosstermBackend<W>>;


 fn main()->Result<(),MyError>{

     // 定义汉宁窗函数
     fn hanning_window(length: usize) -> Vec<f32> {
         (0..length).map(|i| 0.5 - 0.5 * (2.0 * PI * i as f32 / (length - 1) as f32).cos()).collect()
     }

     // let mut app=App::new();
   // app.run().await?;
  //  runner(app).await?;

    // 设置音频输出流和解码器
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let file = File::open("music/music1.mp3").expect("文件路径有问题");
    let buf_reader = BufReader::new(file);
    let mut source = Decoder::new(buf_reader).unwrap();
     let sink = Sink::try_new(&stream_handle).unwrap();
    //sink.append(source);

    //FFT 设置
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(1024); // 假设使用 1024 点 FFT
    let fft_buffer = Arc::new(Mutex::new(vec![Complex::new(0.0, 0.0); 1024]));
    let fft_buffer_clone = Arc::clone(&fft_buffer);
    //stream_handle.play_raw(source.convert_samples::<f32>()).unwrap();
    // 创建缓冲区，用于存储采集的音频数据
     let mut audio_buffer = Vec::<f32>::with_capacity(1024);// 使用 with_capacity 来避免额外分配
     let mut window = hanning_window(1024);
     for sample in source{

         // 将样本与汉宁窗进行逐元素相乘
         let windowed_sample = (sample as f32) * 1.0;
       //  println!("window[0]: {}, windowed_sample: {}", window[0], windowed_sample);
         audio_buffer.push(windowed_sample );

         if audio_buffer.len() >= 1024 {
             let mut complex_samples: Vec<Complex<f32>> = audio_buffer.iter().map(|&sample| Complex::new(sample, 0.0)).collect();
             fft.process(complex_samples.as_mut_slice());
             let magnitude_buffer: Vec<i32> = complex_samples.iter()
                 .map(|&c| c.norm() as i32) // 计算模并转换为 i32
                 .collect();
             println!("模是: {:?}", magnitude_buffer);
             // 清空缓冲区，准备下一轮数据
             audio_buffer.clear();
         }
     }

    // 播放音频并实时捕获样本
    // stream_handle.play_raw(source.convert_samples::<f32>().map(move |sample| {
    //
    //     let mut fft_buffer = fft_buffer_clone.lock().unwrap();
    //     fft_buffer.rotate_left(1);
    //     fft_buffer[1023] = Complex::new(sample, 0.0); // 假设是单声道音频
    //
    //     // FFT 数据准备好后执行 FFT
    //     if fft_buffer.iter().filter(|&&c| c != Complex::new(0.0, 0.0)).count() == 1024 {
    //         let mut output_buffer = vec![Complex::new(0.0, 0.0); 1024];
    //         fft.process(&mut output_buffer);
    //
    //         // 在这里处理 FFT 结果，比如更新音频可视化等
    //         // 假设 output_buffer 包含 FFT 的输出
    //         let output_buffer: Vec<Complex<f32>> = vec![Complex::new(0.0, 0.0); 1024]; // 示例数据
    //
    //         // 转换为幅度并存储为 i32
    //         let magnitude_buffer: Vec<i32> = output_buffer.iter()
    //             .map(|&c| c.norm() as i32) // 计算模并转换为 i32
    //             .collect();
    //         println!("模是: {:?}",magnitude_buffer);
    //     }
    //
    //     sample // 确保音频继续播放
    // })).unwrap();
    std::thread::sleep(Duration::from_secs(20.to_u64().unwrap()));//等待播放完成
    Ok(())
}
//-----------------测试日志滚动-----------------------
// use std::{error::Error, io, time::{Duration, Instant}, vec};
// use std::io::BufRead;
//
// use crossterm::{
//     event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
//     execute,
//     terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
// };
// use ratatui::{prelude::*, widgets::*};
//
// struct StatefulList<T> {
//     state: ListState,
//     items: Vec<T>,
// }
//
// impl<T> StatefulList<T> {
//     fn with_items(items: Vec<T>) -> StatefulList<T> {
//         StatefulList {
//             state: ListState::default(),
//             items,
//         }
//     }
//
//     fn next(&mut self) {
//         let i = match self.state.selected() {
//             Some(i) => {
//                 if i >= self.items.len() - 1 {
//                     0
//                 } else {
//                     i + 1
//                 }
//             }
//             None => 0,
//         };
//         self.state.select(Some(i));
//     }
//
//     fn previous(&mut self) {
//         let i = match self.state.selected() {
//             Some(i) => {
//                 if i == 0 {
//                     self.items.len() - 1
//                 } else {
//                     i - 1
//                 }
//             }
//             None => 0,
//         };
//         self.state.select(Some(i));
//     }
//
//     fn unselect(&mut self) {
//         self.state.select(None);
//     }
// }
//
// /// This struct holds the current state of the app. In particular, it has the `items` field which is
// /// a wrapper around `ListState`. Keeping track of the items state let us render the associated
// /// widget with its state and have access to features such as natural scrolling.
// ///
// /// Check the event handling at the bottom to see how to change the state on incoming events.
// /// Check the drawing logic for items on how to specify the highlighting style for selected items.
// struct App<'a> {
//     items: StatefulList<(&'a str, usize)>,
//     events: Vec<(&'a str, &'a str)>,
//     log:Vec<String>,
// }
//
// impl<'a> App<'a> {
//     fn new() -> App<'a> {
//         App {
//             items: StatefulList::with_items(vec![
//                 ("Item0", 1),
//                 ("Item1", 2),
//                 ("Item2", 1),
//                 ("Item3", 3),
//                 ("Item4", 1),
//                 ("Item5", 4),
//                 ("Item6", 1),
//                 ("Item7", 3),
//                 ("Item8", 1),
//                 ("Item9", 6),
//                 ("Item10", 1),
//                 ("Item11", 3),
//                 ("Item12", 1),
//                 ("Item13", 2),
//                 ("Item14", 1),
//                 ("Item15", 1),
//                 ("Item16", 4),
//                 ("Item17", 1),
//                 ("Item18", 5),
//                 ("Item19", 4),
//                 ("Item20", 1),
//                 ("Item21", 2),
//                 ("Item22", 1),
//                 ("Item23", 3),
//                 ("Item24", 1),
//             ]),
//             events: vec![
//                 ("Event1", "INFO"),
//                 ("Event2", "INFO"),
//                 ("Event3", "CRITICAL"),
//                 ("Event4", "ERROR"),
//                 ("Event5", "INFO"),
//                 ("Event6", "INFO"),
//                 ("Event7", "WARNING"),
//                 ("Event8", "INFO"),
//                 ("Event9", "INFO"),
//                 ("Event10", "INFO"),
//                 ("Event11", "CRITICAL"),
//                 ("Event12", "INFO"),
//                 ("Event13", "INFO"),
//                 ("Event14", "INFO"),
//                 ("Event15", "INFO"),
//                 ("Event16", "INFO"),
//                 ("Event17", "ERROR"),
//                 ("Event18", "ERROR"),
//                 ("Event19", "INFO"),
//                 ("Event20", "INFO"),
//                 ("Event21", "WARNING"),
//                 ("Event22", "INFO"),
//                 ("Event23", "INFO"),
//                 ("Event24", "WARNING"),
//                 ("Event25", "INFO"),
//                 ("Event26", "INFO"),
//             ],
//             log: vec![
//                 ("Event1".to_string()),
//                 ("Event2".to_string()),
//                 ("Event3".to_string()),
//                 ("Event4".to_string()),
//                 ("Event5".to_string()),
//                 ("Event6".to_string()),
//                 ("Event7".to_string()),
//                 ("Event8".to_string()),
//                 ("Event9".to_string()),
//                 ("Event10".to_string()),
//                 ("Event11".to_string()),
//                 ("Event12".to_string()),
//                 ("Event13".to_string()),
//                 ("Event14".to_string()),
//                 ("Event15".to_string()),
//                 ("Event16".to_string()),
//                 ("Event17".to_string()),
//                 ("Event18".to_string()),
//                 ("Event19".to_string()),
//                 ("Event20".to_string()),
//                 ("Event21".to_string()),
//                 ("Event22".to_string()),
//                 ("Event23".to_string()),
//                 ("Event24".to_string()),
//             ],
//         }
//     }
//
//     /// Rotate through the event list.
//     /// This only exists to simulate some kind of "progress"
//     fn on_tick(&mut self) {
//         let event = self.events.remove(0);
//         self.events.push(event);
//     }
// }
//
// fn main() -> Result<(), Box<dyn Error>> {
//     // setup terminal
//     enable_raw_mode()?;
//     let mut stdout = io::stdout();
//     execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
//     let backend = CrosstermBackend::new(stdout);
//     let mut terminal = Terminal::new(backend)?;
//
//     // create app and run it
//     let tick_rate = Duration::from_millis(250);
//     let app = App::new();
//     let res = run_app(&mut terminal, app, tick_rate);
//
//     // restore terminal
//     disable_raw_mode()?;
//     execute!(
//         terminal.backend_mut(),
//         LeaveAlternateScreen,
//         DisableMouseCapture
//     )?;
//     terminal.show_cursor()?;
//
//     if let Err(err) = res {
//         println!("{err:?}");
//     }
//
//     Ok(())
// }
//
// fn run_app<B: Backend>(
//     terminal: &mut Terminal<B>,
//     mut app: App,
//     tick_rate: Duration,
// ) -> io::Result<()> {
//     let mut last_tick = Instant::now();
//     loop {
//         terminal.draw(|f| ui(f, &mut app))?;
//
//         let timeout = tick_rate.saturating_sub(last_tick.elapsed());
//         if crossterm::event::poll(timeout)? {
//             if let Event::Key(key) = event::read()? {
//                 if key.kind == KeyEventKind::Press {
//                     match key.code {
//                         KeyCode::Char('q') => return Ok(()),
//                         KeyCode::Left | KeyCode::Char('h') => app.items.unselect(),
//                         KeyCode::Down | KeyCode::Char('j') => app.items.next(),
//                         KeyCode::Up | KeyCode::Char('k') => app.items.previous(),
//                         _ => {}
//                     }
//                 }
//             }
//         }
//         if last_tick.elapsed() >= tick_rate {
//             app.on_tick();
//             last_tick = Instant::now();
//         }
//     }
// }
//
// fn ui(f: &mut Frame, app: &mut App) {
//     // Create two chunks with equal horizontal screen space
//     let chunks = Layout::default()
//         .direction(Direction::Horizontal)
//         .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
//         .split(f.size());
//
//     // Iterate through all elements in the `items` app and append some debug text to it.
//     let items: Vec<ListItem> = app
//         .items
//         .items
//         .iter()
//         .map(|i| {
//             let mut lines = vec![Line::from(i.0)];
//             for _ in 0..i.1 {
//                 lines.push(
//                     "Lorem ipsum dolor sit amet, consectetur adipiscing elit."
//                         .italic()
//                         .into(),
//                 );
//             }
//             ListItem::new(lines).style(Style::default().fg(Color::Black).bg(Color::White))
//         })
//         .collect();
//
//     // Create a List from all list items and highlight the currently selected one
//     let items = List::new(items)
//         .block(Block::default().borders(Borders::ALL).title("List"))
//         .highlight_style(
//             Style::default()
//                 .bg(Color::LightGreen)
//                 .add_modifier(Modifier::BOLD),
//         )
//         .highlight_symbol(">> ");
//
//     // We can now render the item list
//     f.render_stateful_widget(items, chunks[0], &mut app.items.state);
//
//     // Let's do the same for the events.
//     // The event list doesn't have any state and only displays the current state of the list.
//     let events: Vec<ListItem> = app
//         .events
//         .iter()
//         .rev()
//         .map(|&(event, level)| {
//             // Colorcode the level depending on its type
//             let s = match level {
//                 "CRITICAL" => Style::default().fg(Color::Red),
//                 "ERROR" => Style::default().fg(Color::Magenta),
//                 "WARNING" => Style::default().fg(Color::Yellow),
//                 "INFO" => Style::default().fg(Color::Blue),
//                 _ => Style::default(),
//             };
//             // Add a example datetime and apply proper spacing between them
//             let header = Line::from(vec![
//                 Span::styled(format!("{level:<9}"), s),
//                 " ".into(),
//                 "2020-01-01 10:00:00".italic(),
//             ]);
//             // The event gets its own line
//             let log = Line::from(vec![event.into()]);
//
//             // Here several things happen:
//             // 1. Add a `---` spacing line above the final list entry
//             // 2. Add the Level + datetime
//             // 3. Add a spacer line
//             // 4. Add the actual event
//             ListItem::new(vec![
//                 Line::from("-".repeat(chunks[1].width as usize)),
//                 header,
//                 Line::from(""),
//                 log,
//             ])
//         })
//         .collect();
//   //  let lines: Vec<String> = app.events.lines().map(|line| line.to_string()).collect();//将string 转化为vec
//     let mut list_item:Vec<ListItem>=app.log.iter().rev().map(
//         |event|{ListItem::new(vec![Line::from(vec![event.into()])])}
//     ).collect();
//
//     let events_list = List::new(list_item)
//         .block(Block::default().borders(Borders::ALL).title("List"))
//         .direction(ListDirection::BottomToTop);
//     f.render_widget(events_list, chunks[1]);
// }