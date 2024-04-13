use std::time::Duration;
use std::fs;
use std::path::{Path, PathBuf};
use ratatui::widgets::ListState;
use crate::lyric::{Lyric, LyricController};

//音频集合
#[derive(Debug, Clone)]
pub struct SoundsList{
    pub sounds: Vec<Sound>,
    pub item_status: ListState,//选中状态
    pub last_selected: Option<usize>,//是否有最新选中
}

impl SoundsList{
    //设置路径，获取文件数据
    pub fn set_path(file_path: String)->Self{
        let mut sounds = Vec::new();
        //2个pathbuf,一个放音频路径，一个放歌词路径
        let mut sound_map: std::collections::HashMap<String, (PathBuf, PathBuf)> = std::collections::HashMap::new();

        // Read the directory
        for entry in fs::read_dir(&Path::new(file_path.as_str())).expect("读取路径下的文件失败") {
            let entry = entry.expect("获取文件路径失败");
            println!("entry is {entry:?}");
            let path = entry.path();

            // Check if it's a file and then process
            if path.is_file() {
                if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                        match ext {
                            "mp3" => {
                                let entry = sound_map.entry(stem.to_string()).or_insert((PathBuf::new(), PathBuf::new()));
                                entry.0 = path.clone();
                            //    println!("entry.0 is {:?}",entry.0);
                            },
                            "lrc" => {
                                let entry = sound_map.entry(stem.to_string()).or_insert((PathBuf::new(), PathBuf::new()));
                                entry.1 = path.clone();
                              //  println!("entry.1 is {:?}",entry.1);
                            },
                            _ => {}
                        }
                    }
                }
            }
        }
       // println!("sounds_map is {:?}",sound_map);
        // Convert the map into a vector of Sound structs
        for (name, paths) in sound_map {
                sounds.push(Sound {
                    id: 0, // ID would typically be set by a database or some other method
                    name: name.clone(),
                    total_duration: Duration::new(0, 0), // Placeholder, would need to be calculated
                    lyric: Lyric::new(), // Placeholder, would need to read the file or similar
                    sound_path: paths.0.to_str().unwrap_or_default().to_string(),
                    lyric_path: paths.1.to_str().unwrap_or_default().to_string(),
                });

        }
       // println!("sounds is {:?}",sounds);
        Self{sounds,
            item_status:ListState::default(),
            last_selected: None,}
    }
    //获取音频信息
    pub fn get_sound_infor(&self,id:u32)->Sound{
        let index = id as usize;
        self.sounds[index].clone()
    }
    pub fn get_sound_name_list(&self)->Vec<(u32,String)>{
        let mut sound_name_list=Vec::new();
        for sound in &self.sounds {
            sound_name_list.push((sound.id ,sound.name.clone()));
        }
        sound_name_list
    }

}

impl Default for SoundsList{
    //默认
    fn default() -> Self {
        Self{sounds:vec!(Sound::new()),
            item_status:ListState::default(),
            last_selected: None,
        }
    }
}

//单个音频文件对象
#[derive(Debug,Clone)]
pub struct Sound{
    pub id:u32,
    pub name:String,
    pub total_duration:Duration,
    pub lyric:Lyric,
    pub sound_path:String,
    pub lyric_path:String,
}
impl Sound{
    pub fn new()->Self{
        Self::default()
    }
    //获取音频的时长
    fn get_total_durtation(&self)->Duration{

        let path = Path::new(&self.sound_path);
        let time = mp3_duration::from_path(&path).expect("获取时间失败");
        time
    }
    //获取歌词
    fn get_lyric(&self)->Vec<Lyric>{
        let mut lyric_controller =LyricController::new(self.lyric_path.clone());
        lyric_controller.get_file().expect("获取歌词信息失败");
        lyric_controller.inital_lyric()
    }
    //获取音频路径
    fn get_sound_path(&self)->String{
       self.sound_path.clone()
    }

}
impl Default for Sound {
    fn default() -> Self {
            Self{
                 id:0,
                 name:String::from("示例数据"),
                 total_duration:Duration::from_secs(20),
                 lyric:Lyric::new(),
                 sound_path:String::from("示例路径"),
                 lyric_path:String::from("示例路径"),
            }
    }

}