use crate::backtrace::Frame;
use crate::{Result, StdError};
use crate::term::Colorized;
use std::any::TypeId;
use std::convert::From;
use std::fmt::{self, Debug, Display, Formatter};

static ERROR_TYPE: &str = "witcher::Error";
static STDERROR_TYPE: &str = "StdError";
static LONG_ERROR_TYPE: &str = "witcher::error::Error";

/// `Error` is a wrapper around lower level error types to provide additional context.
/// 
/// `Error` provides the following benefits
///  - ensures a backtrace will be taken at the earliest opportunity
///  - ensures that the error type is threadsafe and has a static lifetime
///  - provides matching on error types
/// 
/// Context comes in two forms. First every time an error is wrapped you have the
/// opportunity to add an additional message. Finally a simplified stack trace is
/// automatically provided that narrows in on your actual code ignoring the wind up
/// and wind down that resides in the Rust std libraries and other dependencies
/// allowing you to focus on your code.
pub struct Error {

    // Error messages
    msg: String,

    // Original error type and name
    type_id: TypeId,
    type_name: String,

    // Backtrace for the error
    backtrace: Vec<Frame>,

    // Inner wrapped error
    inner: Option<Box<dyn StdError + Send + Sync + 'static>>,
}
impl Error {

    /// Create a new error instance using generics.
    /// 
    pub fn new<M>(msg: M) -> Result<()>
    where 
        M: Display + Debug + Send + Sync + 'static
    {
        Err(Error {
            msg: format!("{}", msg),
            type_id: TypeId::of::<Error>(),
            type_name: String::from(ERROR_TYPE),
            backtrace: crate::backtrace::new(),
            inner: None,
        })
    }

    /// Wrap the given error and include a contextual message for the error.
    ///
    pub fn wrap<E, M>(err: E, msg: M) -> Result<()>
    where
        E: StdError + Send + Sync + 'static,
        M: Display + Send + Sync + 'static,
    {
        let type_id = TypeId::of::<E>();
        let backtrace = crate::backtrace::new();

        // Filter wrapped backtrace to remove duplicate entries from current
        if type_id == TypeId::of::<Error>() {
            //backtrace = backtrace.iter().filter().collect();
        }

        Err(Error {
            msg: format!("{}", msg),
            type_id: type_id,
            type_name: Error::name(&err),
            backtrace: backtrace,
            inner: Some(Box::new(err)),
        })
    }

    /// Wrap the given boxed error and include a contextual message for the error.
    ///
    pub fn wrap_box<M>(err: Box<dyn StdError + Send + Sync + 'static>, msg: M) -> Result<()>
    where
        M: Display + Send + Sync + 'static,
    {
        Err(Error {
            msg: format!("{}", msg),
            type_id: TypeId::of::<dyn StdError>(),
            type_name: String::from(STDERROR_TYPE),
            backtrace: crate::backtrace::new(),
            inner: Some(err),
        })
    }

    /// Extract the name of the given error type and perform some clean up on the type
    fn name<T>(_: T) -> String {
        let mut name = format!("{}", std::any::type_name::<T>());

        // Strip off prefixes
        if name.starts_with('&') {
            name = String::from(name.trim_start_matches('&'));
        }

        // Strip off suffixes
        name = String::from(name.split("<").next().unwrap_or("<unknown>"));

        // Hide full Error path
        if name == LONG_ERROR_TYPE {
            name = String::from(ERROR_TYPE);
        }

        name
    }

    /// Returns `true` if the wrapped error type is the same as `E`
    pub fn is<E>(&self) -> bool
    where
        E: StdError + Send + Sync + 'static,
    {
        TypeId::of::<E>() == self.type_id
    }

    // Common implementation for displaying error.
    // A lifetime needs called out here for the frames and the frame references
    // to reassure Rust that they will exist long enough to get the data needed.
    fn write_err<'a, T>(&self, f: &mut Formatter<'_>, frames: T) -> fmt::Result
    where 
        T: Iterator<Item = &'a Frame>,
    {
        let c = Colorized::new();
        let mut cause: Option<String> = None;

        // Print inner error first
        if let Some(inner) = (self as &dyn StdError).source() {
            if self.type_id == TypeId::of::<Error>() {
                Display::fmt(&inner, f)?;
            } else {
                cause = Some(format!(" cause: {}: {}", c.red(&self.type_name), c.red(&inner)));
            }
        }

        // Print out this error
        writeln!(f, " error: {}", c.red(&self.msg))?;
        if let Some(root_cause) = cause {
            writeln!(f, "{}", root_cause)?;
        }
        self.write_frames(f, &c, frames)?;
        Ok(())
    }

    // Write out the frames
    fn write_frames<'a, T>(&self, f: &mut Formatter<'_>, c: &Colorized, frames: T) -> fmt::Result
    where
        T: Iterator<Item = &'a Frame>,
    {
        for frame in frames {
            // Add the symbol and file information
            writeln!(f, "symbol: {}", c.cyan(&frame.symbol))?;
            write!(f, "    at: {}", frame.filename)?;

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

// External trait implementations
// -------------------------------------------------------------------------------------------------
impl StdError for Error
{
    fn source(&self) -> Option<&(dyn StdError + 'static)>
    {
        match &self.inner {
            Some(x) => Some(&**x),
            None => None,
        }
    }
}

/// Provides the same formatting for output as Display but includes the fullstack trace.
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.write_err(f, self.backtrace.iter())
    }
}

/// Provides formatting for output with frames filtered to just target code
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.write_err(f, self.backtrace.iter().filter(|x| !x.is_dependency()))
    }
}

// /// Converts to Error from boxed std error
// impl From<Box<dyn StdError + Send + Sync + 'static>> for Error {
//     fn from(err: Box<dyn StdError + Send + Sync + 'static>) -> Self {
//         Error::wrap(err, "").unwrap_err()
//     }
// }

// impl From<Error> for Box<dyn StdError + Send + 'static> {
//     fn from(error: Error) -> Self {
//         Box::<dyn StdError + Send + Sync>::from(error)
//     }
// }

// impl From<Error> for Box<dyn StdError + 'static> {
//     fn from(error: Error) -> Self {
//         Box::<dyn StdError + Send + Sync>::from(error)
//     }
// }

// /// Convert to a reference 
// impl AsRef<dyn StdError + Send + Sync> for Error {
//     fn as_ref(&self) -> &(dyn StdError + Send + Sync + 'static) {
//         &**self
//     }
// }
// impl AsRef<dyn StdError> for Error {
//     fn as_ref(&self) -> &(dyn StdError + 'static) {
//         &**self
//     }
// }

// // Unit tests
// // -------------------------------------------------------------------------------------------------
// #[cfg(test)]
// mod tests {
//     use super::*;
    
//     fn io_error() -> crate::Result<()> {
//         Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))?
//     }

//     #[test]
//     fn test_new() {
//         assert_eq!(String::from("foo"), Error::new("foo").msg);
//         //assert_eq!(String::from("foo"), Error::new(String::from("foo")).msg);
//         //assert_eq!(String::from("foo"), Error::new(Path::new("foo").display()).msg);
//     }

//     #[test]
//     fn test_conversion_from_io_error() {
//         let err = io_error().unwrap_err();
//         // if let Some(e) = err.downcast_ref::<std::io::Error>() {
            
//         // }
//         assert_eq!("Custom { kind: Other, error: \"oh no!\" }", err.msg);
//         //assert_eq!(err.msg, format!("{:?}", err.wrapped.unwrap()));
//         //println!("Failed: {}", err);
//     }
// }