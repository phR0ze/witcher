use crate::backtrace::Frame;
use crate::{Result, StdError};
use crate::term::Colorized;
use std::any::TypeId;
use std::convert::From;
use std::fmt::{self, Debug, Display, Formatter};

static ERROR_TYPE: &str = "witcher::Error";
static STDERROR_TYPE: &str = "StdError";
static LONG_ERROR_TYPE: &str = "witcher::error::Error";

/// `Error` is a wrapper providing additional context and chaining of errors.
/// 
/// `Error` provides the following benefits
///  - ensures a backtrace will be taken at the earliest opportunity
///  - ensures that the error type is threadsafe and has a static lifetime
///  - provides matching on inner error types
/// 
/// Context comes in two forms. First every time an error is wrapped you have the
/// opportunity to add an additional message. Finally a simplified stack trace is
/// automatically provided that narrows in on your actual code ignoring the wind up
/// and wind down that resides in the Rust std libraries and other dependencies
/// allowing you to focus on your code.
/// 
/// Saftey: data layout ensured to be consistent with repr(C) for raw conversions.
pub struct Error
{
    // Error message which will either be additional context for the inner error
    // or in the case where this error was created from `new` will be the only
    // error message.
    msg: String,

    // Type id and type name here will refer to the inner error in the case where
    // inner error is Some and is an external type else it will be `Error`. 
    type_id: TypeId,
    type_name: String,

    backtrace: Vec<Frame>,

    // The original error in the case where we're wrapping an external error or
    // an `Error` in the case where we're wrapping another `Error`.
    inner: Option<Box<dyn StdError + Send + Sync + 'static>>,
}
impl Error
{
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
        })
    }

    /// Wrap the given error and include a contextual message for the error.
    ///
    pub fn wrap<E, M>(err: E, msg: M) -> Result<()>
    where
        E: StdError + Send + Sync + 'static,
        M: Display + Send + Sync + 'static,
    {
        Err(Self {
            msg: format!("{}", msg),
            type_id: TypeId::of::<E>(),
            type_name: Error::name(&err),
            backtrace: crate::backtrace::new(),
            inner: Some(Box::new(err)),
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
            inner: Some(err.into()),
        })
    }

    /// Extract the name of the given error type and perform some clean up on the type
    fn name<T>(_: T) -> String {
        let mut name = format!("{}", std::any::type_name::<T>());

        // Strip off prefixes
        if name.starts_with('&') {
            name = String::from(name.trim_start_matches('&'));
        }

        if name.starts_with("dyn ") {
            name = String::from(name.trim_start_matches("dyn "));
        }

        // Strip off suffixes
        name = String::from(name.split("<").next().unwrap_or("<unknown>"));

        // Hide full Error path
        if name == LONG_ERROR_TYPE {
            name = String::from(ERROR_TYPE);
        }

        name
    }

    // Common implementation for displaying error.
    // A lifetime needs called out here for the frames and the frame references
    // to reassure Rust that they will exist long enough to get the data needed.
    fn write(&self, f: &mut Formatter<'_>, fullstack: bool) -> fmt::Result
    {
        let c = Colorized::new();

        // Push all `Error` instances to a vec then reverse
        let mut errors: Vec<&Error> = Vec::new();
        let mut source = (self as &(dyn StdError + 'static)).source();
        errors.push(self);
        while let Some(stderr_ref) = source {
            if let Some(err) = stderr_ref.downcast_ref::<Error>() {
                errors.push(err);
                source = stderr_ref.source();
            } else {
                break;
            }
        }
        errors = errors.into_iter().rev().collect();

        // Pop them back off LIFO style
        let len = errors.len();
        for (i, err) in errors.iter().enumerate() {
            let mut parent: Option<&Error> = None;
            if i + 1 < len {
                parent = Some(errors[i+1]);
            }

            // Write out the error wrapper
            writeln!(f, " error: {}", c.red(&err.msg))?;

            // Write out any std errors in order
            if i == 0 {
                if let Some(stderr) = (*err as &(dyn StdError + 'static)).source() {
                    err.write_std(f, &c, stderr)?;
                }
            }

            // Write out the frames minus those in the wrapping error
            err.write_frames(f, &c, err, parent, fullstack)?;
        }
        Ok(())
    }

    // Write out external errors
    fn write_std(&self, f: &mut Formatter<'_>, c: &Colorized, err: &dyn StdError) -> fmt::Result
    {
        let mut buf = format!(" cause: {}", c.red(err));
        let mut source = err.source();
        while let Some(inner) = source {
            if buf.chars().last().unwrap() != '\n' {
                buf += &"\n";
            }
            buf += &format!(" cause: {}", c.red(inner));
            source = inner.source();
        }
        if buf.chars().last().unwrap() != '\n' {
            buf += &"\n";
        }
        write!(f, "{}", buf)
    }

    fn write_frames(&self, f: &mut Formatter<'_>, c: &Colorized, err: &Error, other: Option<&Error>, fullstack: bool) -> fmt::Result
    {
        let frames: Vec<&Frame> = match fullstack {
            false => {
                let frames: Vec<&Frame> = err.backtrace.iter().filter(|x| !x.is_dependency()).collect();
                match other {
                    Some(parent) => {
                        let len = frames.len();
                        let plen = parent.backtrace.iter().filter(|x| !x.is_dependency()).count();
                        frames.into_iter().take(len - plen).collect::<Vec<&Frame>>()
                    },
                    _ => frames
                }
            },

            // Fullstack `true` means don't filter anything
            _ => err.backtrace.iter().collect()
        };

        for frame in frames {
            writeln!(f, "symbol: {}", c.cyan(&frame.symbol))?;
            write!(f, "    at: {}", frame.filename)?;

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
        self.write(f, true)
    }
}

/// Provides formatting for output with frames filtered to just target code
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        self.write(f, false)
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    // Disable backtrace and colors
    use std::sync::Once;
    static INIT: Once = Once::new();
    pub fn initialize() {
        INIT.call_once(|| {
            env::set_var("COLOR", "0");
            env::set_var("RUST_BACKTRACE", "0");
        });
    }

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
        initialize();
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
        let mut err_str = format!("{}", Error::wrap(err, "wrapped").unwrap_err());
        err_str = err_str.split("rs:").nth(0).unwrap().to_string();
        assert_eq!(" error: wrapped\n cause: cause 1\n cause: cause 2\n cause: cause 3\n", err_str);
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