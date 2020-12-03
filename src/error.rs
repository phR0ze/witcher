use crate::term;
use crate::Error;
use std::{fmt, error};
use colored::*;

impl Error {
    /// Create a new error instance
    /// 
    pub fn new(message: &'static str) -> Self {
        Self { 
            message,
            frames: crate::backtrace::new(),
        }
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.message)?;
        for frame in self.frames.iter() {

            // Add the symbol and file names
            if term::isatty() {
                writeln!(f, "{}", frame.symbol.red().bold())?;
            } else {
                writeln!(f, "{}", frame.symbol)?;
            }
            write!(f, "  at {}", frame.filename)?;

            // Add the line and columen if they exist
            if let Some(line) = frame.lineno {
                write!(f, ":{}", line)?;
                if let Some(column) = frame.column {
                    write!(f, ":{}", column)?;
                }
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}

