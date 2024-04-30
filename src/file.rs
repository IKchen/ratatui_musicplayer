
//该文件已被sounds 文件代替
use std::path::Path;
use ratatui::widgets::ListState;
use tokio::fs;
use tokio::fs::File;
use tokio::io::AsyncReadExt;
use crate::error::MyError;
#[derive(Clone,Debug)]
pub enum ItemStatus{
    Selected,
    UnSelected,
    Playing,
}
#[derive(Clone,Debug)]
pub struct FileItem{
    pub item_name:String,
}
#[derive(Clone,Debug)]
pub struct FileList {
    pub file_path:String,
    pub item_list:Vec<FileItem>,
    pub item_status: ListState,
    pub last_selected: Option<usize>,
}
impl FileItem{
    pub fn new(file_name:String)->Self{
        let item_name=file_name;
        Self{item_name}
    }
}
impl FileList{
    pub fn new()->Self{
        let mut file_path="./music".to_string();
        let mut item_list=Vec::new();
        let item_status = ListState::default();
        let last_selected=None;
        Self{file_path,item_list,item_status,last_selected}
    }
    pub async fn load_filelist(& mut self)->Result<(),MyError>{
        let path_clone=self.file_path.clone();
        let path=Path::new(&path_clone);
         let mut entries = fs::read_dir(path).await?;
            while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            if path.is_file() {
                // 如果是文件，则获取文件名
                if let Some(file_name) = path.file_name().and_then(|name| name.to_str()) {
                    let mut file=file_name.to_string();
                    let file_item=FileItem::new(file);
                    self.item_list.push(file_item);

                    // 异步读取文件内容
                    let mut file = File::open(&path).await?;
                    let mut contents = Vec::new();
                    file.read_to_end(&mut contents).await?;

                    // 打印文件内容的长度（或者做其他处理）
                  //  println!("{} length: {}", file_name, contents.len());
                }
            }
        }
       // println!("Reading file: {:?}", self.item_list);
        Ok(())
    }
    pub fn get_file_list(self)->FileList{
        self
    }//获取文件名列表
}