use serde_json::error::Error as SerdeError;
use std::string::FromUtf8Error;
use std::io::Error;

#[derive(Debug)]
pub enum MyError {
    Utf8Error(FromUtf8Error),
    JsonError(SerdeError),
    StdError(Error),
}

impl From<FromUtf8Error> for MyError {
    fn from(err: FromUtf8Error) -> Self {
        MyError::Utf8Error(err)
    }
}

impl From<SerdeError> for MyError {
    fn from(err: SerdeError) -> Self {
        MyError::JsonError(err)
    }
}

impl From<Error> for MyError {
    fn from(err: Error) -> Self {
        MyError::StdError(err)
    }
}
