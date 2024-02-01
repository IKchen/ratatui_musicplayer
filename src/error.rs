//自定义错误转换

use  std::io::Error;
use tokio::task::JoinError;
use crate::action::Action;

#[derive(Debug)]
pub enum MyError{
    IoError(std::io::Error),//打印失败
    JoinError(tokio::task::JoinError),
    InitializationError,
    TokioSendError(tokio::sync::mpsc::error::SendError<Action>),//线程发送失败
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
// impl From<setglobal> for MyError{
//     fn from(error: tokio::task::JoinError)->MyError{
//         MyError::JoinError(error)
//     }
// }
impl From<tokio::sync::mpsc::error::SendError<Action>> for MyError{
    fn from(error:tokio::sync::mpsc::error::SendError<Action>)->MyError{
        MyError::TokioSendError(error)
    }
}