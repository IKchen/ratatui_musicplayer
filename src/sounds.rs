use std::time::Duration;
use std::{fs, vec};
use std::path::{Path, PathBuf};
use futures::future::ok;
use ratatui::widgets::ListState;
use crate::lyric::{Lyric, LyricController};

//音频集合
#[derive(Debug, Clone)]
pub struct SoundsList{
    pub sounds: Vec<Sound>,
    pub item_status: ListState,//选中状态
    pub last_selected:  Option<usize>,//上次选中
    pub next_selected: Option<usize>,//是否有最新选中,用下次预备播放
    pub playing_item_id:Option<usize>,//正要播放
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
          //  println!("entry is {entry:?}");
            let path = entry.path();

            // Check if it's a file and then process  这里把所有的file 传入sound map了，后面实际上我们只需要音频文件在sounds 里面
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
        for (id,(name, paths)) in sound_map.iter().enumerate() {
           if !paths.0.as_os_str().is_empty(){ //如果mp3路径不为空，就push到sound数组里面
               sounds.push(Sound {
                   id, // ID would typically be set by a database or some other method
                   name: name.clone(),
                   total_duration: Duration::new(0, 0), // Placeholder, would need to be calculated
                   lyric: Lyric::new(), // Placeholder, would need to read the file or similar
                   sound_path: paths.0.to_str().unwrap_or_default().to_string(),
                   lyric_path: paths.1.to_str().unwrap_or_default().to_string(),
               });
           }

        }

        Self{sounds,
            item_status:ListState::default(),
            last_selected: None,playing_item_id:None ,next_selected:None}
    }
    //获取当前播放音频名称
    pub fn get_playingsound_name(&self)->String{
        if let Some(index)=self.playing_item_id{
            self.sounds[index].name.clone()
        } else {
            String::from("没有选中任何音乐")
        }

    }
    //获取当前播放音频路径
    pub fn get_playingsound_path(&self)->String{

        if let Some(index)=self.playing_item_id{
            self.sounds[index].sound_path.clone()
        } else {
            String::from("没有选中任何音乐")
        }
    }
    //获取当前播放歌词路径
    pub fn get_playingsound_lyric(&self)->Vec<Lyric>{

        if let Some(index)=self.playing_item_id{
        //    println!("index is {index:?},path is {:?}",self.sounds[index].lyric_path);
            self.sounds[index].get_lyric()
        } else {
           vec![Lyric::new()]
        }
    }
    //获取当前播放音频总时长
    pub fn get_playingsound_total_durtation(&self)->Duration{

        if let Some(index)=self.playing_item_id{
            self.sounds[index].get_total_durtation()
        } else {
            Duration::from_secs(0)
        }
    }
    //获取音频列表
    pub fn get_sound_name_list(&self)->Vec<(usize,String)>{
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
            playing_item_id:None,
            next_selected:None
        }
    }
}

//单个音频文件对象
#[derive(Debug,Clone)]
pub struct Sound{
    pub id:usize,
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
        match mp3_duration::from_path(&path) {
            Ok(duration) => duration,
            Err(err) => {
                // 打印错误信息
                println!("获取时间失败, path is {:?}, error: {:?}", path, err);
                // 返回一个默认的 Duration，比如 0
                Duration::new(0, 0)

            }
        }
    }
    //获取歌词
    fn get_lyric(&self)->Vec<Lyric>{
     //   println!("path is {:?}",self.lyric_path);
      if self.lyric_path!=""{
          let mut lyric_controller =LyricController::new(self.lyric_path.clone());
          lyric_controller.get_lyric();
          lyric_controller.inital_lyric()
      }else {
          vec![Lyric::new()]
      }

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
#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::BufReader;
    use crate::event::Event;
    use crate::sounds::SoundsList;

    #[test]
    fn test_sound_lyric_path() {
        let soundlist=SoundsList::set_path("music".to_string());
        for sound in soundlist.sounds{
            if sound.sound_path=="music/国际歌.mp3"{
                assert_eq!("music/国际歌-MusicEnc.lrc", sound.lyric_path);
            }
        }

    }
}