use  std::io::Error;
use tokio::task::JoinError;

#[derive(Debug)]
pub enum MyError{
    IoError(std::io::Error),
    JoinError(tokio::task::JoinError),
}
impl  From<std::io::Error> for MyError{
    fn from(error: std::io::Error)->MyError{
        MyError::IoError(error)
    }
}
impl From<JoinError> for MyError{
    fn from(error: tokio::task::JoinError)->MyError{
        MyError::JoinError(error)
    }
}