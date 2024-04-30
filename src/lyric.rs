
use std::ffi::OsStr;
use std::fs;
use std::fs::File;
use std::io::Read;
use crate::error::MyError;
#[derive(Clone,Debug)]
// 单句歌词和时间标记
pub struct Lyric{
    pub lyric:String,
    pub time:String,
}
#[derive(Clone,Debug)]
pub struct LyricController{
    pub filestring:String,
    pub lyric:Vec<Lyric>,
    pub time:String,
    pub path:String
}

impl Lyric{
    pub fn new()->Self{
        Self{lyric:String::from("暂无歌词"),time:String::from("[00:00]")}
    }
}
impl LyricController {
    pub fn new(path:String)->Self{
        let mut path=path;
        let mut lyric=vec![Lyric::new()];
        let mut time=String::new();
        let mut filestring=String::new();//歌词string
        Self{path,lyric,time,filestring}
    }
    //获取文件的异步实现
    // pub async fn get_file(&mut self)->Result<(),MyError>{
    //     let mut entries = fs::read_dir("./music").await?;
    //     while let Some(entry) = entries.next_entry().await? {
    //         let path = entry.path();
    //         if path.is_file() {
    //             match path.extension().and_then(OsStr::to_str) { // 转换为 &str 以便比较
    //                 Some("lrc") => {
    //                     File::open(path.clone()).expect("歌词文件打开失败").read_to_string(&mut self.filestring).expect("歌词文件读取失败");
    //                 //    println!("Found lyc file: {:?}", path);
    //                 //    println!("lyric is \n {}",self.filestring);
    //                 }
    //                 Some("mp3") => println!("Found mp3 file: {:?}", path),
    //                 Some(ext) => println!("Found other type of file: {}, extension: {}", path.display(), ext),
    //                 None => println!("File has no extension: {:?}", path),
    //             }
    //         }
    //     }
    //     Ok(())
    // }
    //获取文件的同步实现
    // pub  fn get_lyric_path(&mut self)->Result<(),MyError>{
    //     let mut entries = fs::read_dir("./music").expect("路径未读取成功");
    //     for entry in entries{
    //         let path = entry.expect("根据路径获取文件信息失败").path();
    //         if path.is_file() {
    //             match path.extension().and_then(OsStr::to_str) { // 转换为 &str 以便比较
    //                 Some("lrc") => {
    //                     File::open(path.clone()).expect("歌词文件打开失败").read_to_string(&mut self.filestring).expect("歌词文件读取失败");
    //                     //    println!("Found lyc file: {:?}", path);
    //                     //    println!("lyric is \n {}",self.filestring);
    //                 }
    //                 Some("mp3") =>{} //println!("Found mp3 file: {:?}", path),
    //                 Some(ext) =>{} //println!("Found other type of file: {}, extension: {}", path.display(), ext),
    //                 None =>{}// println!("File has no extension: {:?}", path),
    //             }
    //         }
    //     }
    //     Ok(())
    // }
    pub fn get_lyric(&mut self){

        File::open(self.path.clone()).expect("歌词文件打开失败").read_to_string(&mut self.filestring).expect("歌词文件读取失败");
    }
    pub fn inital_lyric(&mut self)->Vec<Lyric>{
        let lines: Vec<Lyric> = self.filestring.split('\n')
            .filter_map(|line| {
                // 尝试从每一行中提取时间和歌词
                if let Some((time, lyric)) = line.split_once(']') {
                    // 移除时间部分的开头'['
                    let time = time.trim_start_matches('[').to_string();
                    let lyric = lyric.trim().to_string();
                    Some(Lyric { time, lyric })
                } else {
                    None
                }
            })
            .collect();

        // 打印结果，验证Lyric结构体是否正确填充
        // for lyric in lines.iter() {
        //     println!("{} - {}", lyric.time, lyric.lyric);
        // }
        lines
    }

}