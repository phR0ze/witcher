use crate::term;
use colored::*;
use std::error;
use std::fmt::{Display, Debug, Formatter, Result};

/// `Error` is a wrapper around lower level error types to provide additional context.
///
/// All errors implement the std::error::Error trait and so any error can be converted
/// into a Box<dyn std::error:Error>.
///
/// Context comes in two forms. First every time an error is wrapped you have the
/// opportunity to add an additional message. Finally a simplified stack trace is
/// automatically provided that narrows in on your actual code ignoring the wind up
/// and wind down that resides in the Rust std libraries and other dependencies
/// allowing you to focus on your code.
pub struct Error {
    msg: String,
    frames: Vec<crate::backtrace::Frame>,
}
impl Error {
    /// Create a new error instance using generics.
    /// 
    /// Supports any type that implements the trait bounds
    pub fn new<T>(msg: T) -> Self
        where T: Display + Send + Sync + 'static
    {
        Self {
            msg: format!("{}", msg),
            frames: crate::backtrace::new(),
        }
    }
}

// Use default error implementation
impl error::Error for Error {
}

impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        Display::fmt(&self, f)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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

// pub trait Convert {
//     fn err<T>(self) -> Error
//         where T: Display + Send + Sync + 'static;
// }

// impl Convert for &str
// {
//     fn err<T>(self) -> Error
//         where T: Display + Send + Sync + 'static,
//     {
//         Error {
//             msg: String::from("foo"),
//             frames: crate::backtrace::new(),
//         }
//     }
// }

// impl convert::From<error::Error> for Error {
//     fn from(err: error:Error) -> Self {
//         Error{}
//     }
// }
        // Self { 
        //     msg: msg.into(),
        //     frames: crate::backtrace::new(),
        // }
// pub trait ErrorExt {
//     fn into<T>(other: T) -> Error
//         where T: Display + Send + Sync + 'static;
// }

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_new() {
        assert_eq!(String::from("foo"), Error::new("foo").msg);
        //assert_eq!(String::from("foo"), Error::new(String::from("foo")).msg);
        //assert_eq!(String::from("foo"), Error::new(Path::new("foo").display()).msg);
    }
}