use crate::Error;
use std::{fmt, error};

impl Error {
    /// Create a new error instance
    /// 
    pub fn new(message: &'static str) -> Self {
        Self { 
            message,
            backtrace: crate::backtrace::simple(),
        }
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\n{}", self.message, self.backtrace)
    }
}

