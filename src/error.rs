use crate::term;
use colored::*;

/// `Error` is a wrapper around lower level error types to provide additional context.
/// 
/// `Error` provides the following benefits
///  - ensures a backtrace will be taken at the earliest opportunity
///  - ensures that the error type is threadsafe and has a static lifetime
/// 
/// Context comes in two forms. First every time an error is wrapped you have the
/// opportunity to add an additional message. Finally a simplified stack trace is
/// automatically provided that narrows in on your actual code ignoring the wind up
/// and wind down that resides in the Rust std libraries and other dependencies
/// allowing you to focus on your code.
pub struct Error {
    msg: String,
    frames: Vec<crate::backtrace::Frame>,
    //wrapped: Option<Box<dyn std::error::Error + Send + Sync + 'static>>,
}
impl Error {
    /// Create a new error instance using generics.
    /// 
    /// Supports any type that implements the trait bounds
    pub fn new<T>(msg: T) -> Self
    where 
        T: std::fmt::Display + Send + Sync + 'static
    {
        Self {
            msg: format!("{}", msg),
            frames: crate::backtrace::new(),
            //wrapped: None,
        }
    }

    pub fn new_from_err<T>(err: T) -> Self
    where
        T: std::error::Error + Send + Sync + 'static, 
    {
        //let obj: TraitObject = mem::transmute(&error as &dyn StdError);
        Self {
            msg: format!("{}", err),
            frames: crate::backtrace::new(),
        }
    }

    // Common implementation for displaying error.
    // A lifetime needs called out here for the frames and the frame references
    // to reassure Rust that they will exist long enough to get the data needed.
    fn fmt<'a, T>(&self, f: &mut std::fmt::Formatter<'_>, frames: T) -> std::fmt::Result
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
impl std::error::Error for Error {
}

/// Provides the same formatting for output as Display but includes the full
/// stack trace.
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f, self.frames.iter())
    }
}

/// Provides formatting for output with frames filtered to just target code
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.fmt(f, self.frames.iter().filter(|x| !x.is_dependency()))
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self {
            msg: format!("{:?}", err),
            frames: crate::backtrace::new(),
            //wrapped: Some(Box::new(err)),
        }
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    
    fn io_error() -> crate::result::Result<()> {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))?
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
        // if let Some(e) = err.downcast_ref::<std::io::Error>() {
            
        // }
        assert_eq!("Custom { kind: Other, error: \"oh no!\" }", err.msg);
        assert_eq!(err.msg, format!("{:?}", err.wrapped.unwrap()));
        //println!("Failed: {}", err);
    }
}