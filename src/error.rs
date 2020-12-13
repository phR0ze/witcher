use crate::backtrace::Frame;
use crate::{Result, StdError};
use crate::term::Colorized;
use std::any::{Any, TypeId};
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
    inner: Option<Box<dyn Any + Send + Sync + 'static>>,
    error: Option<Box<dyn StdError + Send + Sync + 'static>>,
}
impl Error {

    /// Create a new error instance using generics.
    /// 
    pub fn new<M>(msg: M) -> Result<()>
    where 
        M: Display + Debug + Send + Sync + 'static
    {
        Err(Self {
            msg: format!("{}", msg),
            type_id: TypeId::of::<Error>(),
            type_name: String::from(ERROR_TYPE),
            backtrace: crate::backtrace::new(),
            inner: None,
            error: None,
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
        let type_name = Error::name(&err);

        // Set the appropriate inner value
        let mut inner: Option<Box<dyn Any + Send + Sync + 'static>> = None;
        let mut error: Option<Box<dyn StdError + Send + Sync + 'static>> = None;
        if type_id == TypeId::of::<Error>() {
            inner = Some(Box::new(err));
        } else {
            error = Some(Box::new(err));
        }

        // Filter wrapped backtrace to remove duplicate entries from current
        let backtrace = crate::backtrace::new();
        // (err as (dyn StdError + 'static)).downcast_ref::<Error>();
        //backtrace = backtrace.iter().filter().collect();

        Err(Self {
            msg: format!("{}", msg),
            type_id,
            type_name,
            backtrace,
            inner,
            error,
        })
    }

    /// Wrap the given boxed error and include a contextual message for the error.
    ///
    pub fn wrap_box<M>(err: Box<dyn StdError + Send + Sync + 'static>, msg: M) -> Result<()>
    where
        M: Display + Send + Sync + 'static,
    {
        Err(Self {
            msg: format!("{}", msg),
            type_id: TypeId::of::<dyn StdError>(),
            type_name: String::from(STDERROR_TYPE),
            backtrace: crate::backtrace::new(),
            inner: None,
            error: Some(err),
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

    /// Returns the inner error as Some error reference if it exists else None
    pub fn downcast_ref(&self) -> Option<&Error>
    {
        match self.inner.as_ref() {
            Some(inner) => inner.downcast_ref::<Error>(),
            None => None,
        }
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
        let mut source = (self as &dyn StdError).source();
        if let Some(error) = self.downcast_ref() {
            Display::fmt(error, f)?;
        } else if let Some(error) = source {
            let mut buf = format!(" cause: {}: {}", c.red(&self.type_name), c.red(&error));
            source = error.source();
            while let Some(error) = source {
                if buf.chars().last().unwrap() != '\n' {
                    buf += &"\n";
                }

                // Write out the next error cause
                buf += &format!(" cause: {}", c.red(error));
                source = error.source();
            }
            if buf.chars().last().unwrap() != '\n' {
                buf += &"\n";
            }
            cause = Some(buf);
        }

        // Print outer error
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
        match &self.error {
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

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    struct TestError {
        msg: String,
        inner: Option<Box<TestError>>
    }
    impl Debug for TestError {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "{}", self.msg)
        }
    }
    impl Display for TestError {
        fn fmt(&self, f: &mut Formatter) -> fmt::Result {
            write!(f, "{}", self.msg)
        }
    }
    impl StdError for TestError {
        fn source(&self) -> Option<&(dyn StdError + 'static)>
        {
            match &self.inner {
                Some(x) => Some(x as &dyn StdError),
                None => None,
            }
        }
    }
    
    #[test]
    fn test_chained_cause() {
        env::set_var("COLOR", "0");
        let err = TestError {
            msg: "cause 1".to_string(),
            inner: Some(Box::new(TestError{
                msg: "cause 2".to_string(),
                inner: Some(Box::new(TestError{
                    msg: "cause 3".to_string(),
                    inner: None
                })),
            })),
        };
        assert!(format!("{}", Error::wrap(err, "wrapped").unwrap_err()).starts_with(" error: wrapped\n cause: witcher::error::tests::TestError: cause 1\n cause: cause 2\n cause: cause 3\n"));
    }

//     #[test]
//     fn test_conversion_from_io_error() {
//         let err = io_error().unwrap_err();
//         // if let Some(e) = err.downcast_ref::<std::io::Error>() {
            
//         // }
//         assert_eq!("Custom { kind: Other, error: \"oh no!\" }", err.msg);
//         //assert_eq!(err.msg, format!("{:?}", err.wrapped.unwrap()));
//         //println!("Failed: {}", err);
//     }
}