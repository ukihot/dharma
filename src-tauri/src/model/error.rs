use std::error;
use std::fmt;

#[derive(Debug)]
pub enum MyError {
    TypeError(String),
}

impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            MyError::TypeError(ref s) => write!(f, "Type error: {}", s),
        }
    }
}

impl error::Error for MyError {
    fn description(&self) -> &str {
        match *self {
            MyError::TypeError(ref s) => s,
        }
    }
}
