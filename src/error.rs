use crate::term;
use crate::Error;
use colored::*;
use std::{fmt, error};

impl Error {
    /// Create a new error instance using generics.
    /// 
    /// Supports any type that implements the trait bounds
    pub fn new<T>(msg: T) -> Self
        where T: fmt::Display + Send + Sync + 'static
    {
        Self { 
            msg: format!("{}", msg),
            frames: crate::backtrace::new(),
        }
    }
}

impl error::Error for Error {

}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{}", self.msg)?;
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

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_new() {
        assert_eq!(String::from("foo"), Error::new("foo").msg);
        assert_eq!(String::from("foo"), Error::new(String::from("foo")).msg);
        assert_eq!(String::from("foo"), Error::new(Path::new("foo").display()).msg);
    }
}