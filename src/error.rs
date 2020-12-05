use crate::term;
use colored::*;
use std::{io, error, convert};
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
    wrapped: Option<Box<dyn error::Error + Send + Sync + 'static>>,
}
impl Error {
    /// Create a new error instance using generics.
    /// 
    /// Supports any type that implements the trait bounds
    pub fn new<T>(msg: T) -> Self
    where 
        T: Display + Send + Sync + 'static
    {
        Self {
            msg: format!("{}", msg),
            frames: crate::backtrace::new(),
            wrapped: None,
        }
    }

    // Common implementation for displaying error.
    // A lifetime needs called out here for the frames and the frame references
    // to reassure Rust that they will exist long enough to get the data needed.
    fn fmt<'a, T>(&self, f: &mut Formatter<'_>, frames: T) -> Result
    where 
        T: Iterator<Item = &'a crate::backtrace::Frame>,
    {
        write!(f, "message: ")?;
        if term::isatty() {
            writeln!(f, "{}", self.msg.red().bold())?;
        } else {
            writeln!(f, "{}", self.msg)?;
        }

        for frame in frames {

            // Add the symbol and file names
            write!(f, "   name: ")?;
            if term::isatty() {
                writeln!(f, "{}", frame.symbol.cyan().bold())?;
            } else {
                writeln!(f, "{}", frame.symbol)?;
            }
            write!(f, "     at: {}", frame.filename)?;

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

// Use default error implementation
impl error::Error for Error {
}

/// Provides the same formatting for output as Display but includes the full
/// stack trace.
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.fmt(f, self.frames.iter())
    }
}

/// Provides formatting for output with frames filtered to just target code
impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        self.fmt(f, self.frames.iter().filter(|x| !x.is_dependency()))
    }
}

// From io::Error
impl convert::From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        Self {
            msg: format!("{:?}", err),
            frames: crate::backtrace::new(),
            wrapped: Some(Box::new(err)),
        }
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
    
    fn io_error() -> crate::Result<()> {
        Err(io::Error::new(io::ErrorKind::Other, "oh no!"))?
    }

    #[test]
    fn test_new() {
        assert_eq!(String::from("foo"), Error::new("foo").msg);
        //assert_eq!(String::from("foo"), Error::new(String::from("foo")).msg);
        //assert_eq!(String::from("foo"), Error::new(Path::new("foo").display()).msg);
    }

    #[test]
    fn test_conversion_from_io_error() {
        let err = io_error().unwrap_err();
        // if let Some(e) = err.downcast_ref::<io::Error>() {
            
        // }
        assert_eq!("Custom { kind: Other, error: \"oh no!\" }", err.msg);
        assert_eq!(err.msg, format!("{:?}", err.wrapped.unwrap()));
        //println!("Failed: {}", err);
    }
}