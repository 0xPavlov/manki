use std::{error, fmt};


#[derive(Debug)]
pub(crate) enum Error {
    BadData,
    FileNotFound,
    Default,
}

impl error::Error for Error {   
    
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Error::BadData => write!(f, "Bad data error occurred"),
            Error::FileNotFound => write!(f, "File not found error occurred"),
            Error::Default => write!(f, "Default error"),
        }
    }   
}

pub(crate) struct Logger {
    log: Vec<String>,
}

impl Logger {

    pub(crate) fn log(err: Error) {
        
    }

    pub(crate) fn new() -> Logger {
        Logger { log: Vec::new(), }
    }
}
