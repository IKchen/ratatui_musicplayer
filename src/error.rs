use  std::io::Error;

pub enum MyError{
    IoError(std::io::Error),
}
impl  From<std::io::Error> for MyError{
    fn from(error: std::io::Error)->MyError{
        MyError::IoError(error)
    }
}
