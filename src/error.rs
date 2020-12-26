use crate::backtrace::Frame;
use crate::{Result, StdError};
use gory::*;
use std::convert::From;
use std::fmt::{self, Debug, Display, Formatter};

static ERROR_TYPE: &str = "witcher::Error";
static STDERROR_TYPE: &str = "std::error::Error";
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
pub struct Error {
    // Error message which will either be additional context for the inner error
    // or in the case where this error was created from `new` will be the only
    // error message.
    msg: String,

    // Type name here will refer to the inner error in the case where
    // inner error is Some and is an external type else it will be `Error`.
    type_name: String,

    // Backtrace frames that have been cleaned up
    backtrace: Vec<Frame>,

    // The original error in the case where we're wrapping an external error or
    // an `Error` in the case where we're wrapping another `Error`.
    inner: Option<Box<dyn StdError + Send + Sync + 'static>>,
}
impl Error {
    /// Create a new error instance wrapped in a result
    ///
    pub fn raw(msg: &str) -> Self {
        Self { msg: msg.to_string(), type_name: String::from(ERROR_TYPE), backtrace: crate::backtrace::new(), inner: None }
    }

    /// Wrap the given error and include a contextual message for the error.
    ///
    pub fn wrapr<E>(err: E, msg: &str) -> Self
    where
        E: StdError + Send + Sync + 'static,
    {
        Self { msg: msg.to_string(), type_name: Error::name(&err), backtrace: crate::backtrace::new(), inner: Some(Box::new(err)) }
    }

    /// Create a new error instance wrapped in a result
    ///
    pub fn new<T>(msg: &str) -> Result<T> {
        Err(Error::raw(msg))
    }

    /// Wrap the given error and include a contextual message for the error.
    ///
    pub fn wrap<T, E>(err: E, msg: &str) -> Result<T>
    where
        E: StdError + Send + Sync + 'static,
    {
        Err(Error::wrapr(err, msg))
    }

    /// Return the first external error of the error chain for downcasting.
    /// The intent is that when writing application code there are cases where your more
    /// interested in reacting to an external failure.
    /// If there is no external error then you'll get the last `Error` in the chain.
    pub fn ext(&self) -> &(dyn StdError + 'static) {
        let mut stderr: &(dyn StdError + 'static) = self;
        let mut source = self.source();
        while let Some(err) = source {
            stderr = err;
            if !err.is::<Error>() {
                break;
            }
            source = err.source();
        }
        stderr
    }

    /// Return the last of the error chain for downcasting.
    /// This will follow the chain of source errors down to the last and return it.
    /// If this error is the only error it will be returned instead.
    pub fn last(&self) -> &(dyn StdError + 'static) {
        let mut err: &(dyn StdError + 'static) = self;
        let mut source = self.source();
        while let Some(e) = source {
            err = e;
            source = e.source();
        }
        err
    }

    /// Implemented directly on the `Error` type to reduce casting required
    pub fn is<T: StdError + 'static>(&self) -> bool {
        <dyn StdError + 'static>::is::<T>(self)
    }

    /// Implemented directly on the `Error` type to reduce casting required
    pub fn downcast_ref<T: StdError + 'static>(&self) -> Option<&T> {
        <dyn StdError + 'static>::downcast_ref::<T>(self)
    }

    /// Implemented directly on the `Error` type to reduce casting required
    pub fn downcast_mut<T: StdError + 'static>(&mut self) -> Option<&mut T> {
        <dyn StdError + 'static>::downcast_mut::<T>(self)
    }

    /// Implemented directly on the `Error` type to reduce casting required
    pub fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.as_ref().source()
    }

    /// Extract the name of the given error type and perform some clean up on the type
    fn name<T>(_: T) -> String {
        let mut name = std::any::type_name::<T>().to_string();

        // Strip off prefixes
        if name.starts_with('&') {
            name = String::from(name.trim_start_matches('&'));
        }

        if name.starts_with("dyn ") {
            name = String::from(name.trim_start_matches("dyn "));
        }

        // Strip off suffixes
        name = String::from(name.split('<').next().unwrap_or("<unknown>"));

        // Hide full Error path
        if name == LONG_ERROR_TYPE {
            name = String::from(ERROR_TYPE);
        }

        name
    }

    // Write out external errors
    fn write_std(&self, f: &mut Formatter<'_>, stderr: &dyn StdError) -> fmt::Result {
        let mut buf = format!(" cause: {}: {}", self.type_name.red(), stderr.to_string().red());
        let mut source = stderr.source();
        while let Some(inner) = source {
            if !buf.ends_with('\n') {
                buf += &"\n";
            }
            buf += &format!(" cause: {}: {}", STDERROR_TYPE.red(), inner.to_string().red());
            source = inner.source();
        }
        if !buf.ends_with('\n') {
            buf += &"\n";
        }
        write!(f, "{}", buf)
    }

    fn write_frames(&self, f: &mut Formatter<'_>, parent: Option<&Error>, fullstack: bool) -> fmt::Result {
        let frames: Vec<&Frame> = if !fullstack {
            let frames: Vec<&Frame> = self.backtrace.iter().filter(|x| !x.is_dependency()).collect();
            match parent {
                Some(parent) => {
                    let len = frames.len();
                    let plen = parent.backtrace.iter().filter(|x| !x.is_dependency()).count();
                    frames.into_iter().take(len - plen).collect::<Vec<&Frame>>()
                }
                _ => frames,
            }

        // Fullstack `true` means don't filter anything
        } else {
            self.backtrace.iter().collect()
        };

        let len = frames.len();
        for (i, frame) in frames.iter().enumerate() {
            writeln!(f, "symbol: {}", frame.symbol.cyan())?;
            write!(f, "    at: {}", frame.filename)?;

            if let Some(line) = frame.lineno {
                write!(f, ":{}", line)?;
                if let Some(column) = frame.column {
                    write!(f, ":{}", column)?;
                }
            }
            if i + 1 < len {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

// External trait implementations
// -------------------------------------------------------------------------------------------------

impl AsRef<dyn StdError> for Error {
    fn as_ref(&self) -> &(dyn StdError + 'static) {
        self
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match &self.inner {
            Some(x) => Some(&**x),
            None => None,
        }
    }
}

/// Provides the same formatting for output as Display but includes the fullstack trace.
impl Debug for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let fullstack = f.alternate();

        // Push all `Error` instances to a vec then reverse
        let mut errors: Vec<&Error> = Vec::new();
        let mut source = self.source();
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
            let parent: Option<&Error> = if i + 1 < len { Some(errors[i + 1]) } else { None };

            // Write out the error wrapper
            writeln!(f, " error: {}: {}", ERROR_TYPE.red(), err.msg.red())?;

            // Write out any std errors in order
            if i == 0 {
                if let Some(stderr) = (*err).source() {
                    err.write_std(f, stderr)?;
                }
            }

            // Write out the frames minus those in the wrapping error
            err.write_frames(f, parent, fullstack)?;
            if i + 1 < len {
                writeln!(f)?;
            }
        }
        Ok(())
    }
}

/// Provides formatting for output with frames filtered to just target code
impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        if !f.alternate() {
            return write!(f, "{}", self.msg);
        }

        // Write out more detail
        let mut buf = String::new();
        buf += &format!(" error: {}", self.msg.red());

        // Traverse the whole chain
        let mut source = self.source();
        while let Some(stderr) = source {
            if !buf.ends_with('\n') {
                buf += &"\n";
            }
            buf += &" cause: ".to_string();
            match stderr.downcast_ref::<Error>() {
                Some(err) => buf += &format!("{}", err.msg.red()),
                _ => buf += &format!("{}", stderr.to_string().red()),
            }
            source = stderr.source();
        }
        write!(f, "{}", buf)
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
            env::set_var(gory::TERM_COLOR, "0");
            env::set_var("RUST_BACKTRACE", "0");
        });
    }

    struct TestError {
        msg: String,
        inner: Option<Box<TestError>>,
    }
    #[cfg(not(tarpaulin_include))]
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
        fn source(&self) -> Option<&(dyn StdError + 'static)> {
            match &self.inner {
                Some(x) => Some(x as &dyn StdError),
                None => None,
            }
        }
    }

    #[test]
    fn test_output_levels() {
        initialize();

        // Test standard output
        assert_eq!("wrapped", format!("{}", Error::wrapr(TestError { msg: "cause".to_string(), inner: None }, "wrapped")));

        // Test alternate standard output
        assert_eq!(" error: wrapped\n cause: cause", format!("{:#}", Error::wrapr(TestError { msg: "cause".to_string(), inner: None }, "wrapped")));

        let err = Error::wrapr(TestError { msg: "cause".to_string(), inner: None }, "wrapped");
        assert_eq!(
            " error: witcher::Error: wrapped\n cause: witcher::error::tests::TestError: cause\n",
            format!("{:?}", err).split("symbol").next().unwrap()
        );
        let err = Error::wrapr(
            TestError { msg: "cause".to_string(), inner: Some(Box::new(TestError { msg: "cause2".to_string(), inner: None })) },
            "wrapped",
        );
        assert_eq!(
            " error: witcher::Error: wrapped\n cause: witcher::error::tests::TestError: cause\n cause: std::error::Error: cause2\n",
            format!("{:#?}", err).split("symbol").next().unwrap()
        );
    }

    #[test]
    fn test_chained_cause() {
        initialize();
        let err = TestError {
            msg: "cause 1".to_string(),
            inner: Some(Box::new(TestError {
                msg: "cause 2".to_string(),
                inner: Some(Box::new(TestError { msg: "cause 3".to_string(), inner: None })),
            })),
        };

        assert_eq!(" error: wrapped\n cause: cause 1\n cause: cause 2\n cause: cause 3", format!("{:#}", Error::wrapr(err, "wrapped")));
    }

    #[test]
    fn test_ext_and_last() {
        initialize();
        let err = TestError {
            msg: "cause 1".to_string(),
            inner: Some(Box::new(TestError {
                msg: "cause 2".to_string(),
                inner: Some(Box::new(TestError { msg: "cause 3".to_string(), inner: None })),
            })),
        };
        assert_eq!("foo", Error::wrapr(TestError { msg: "cause 1".to_string(), inner: None }, "foo").to_string());
        assert_eq!("cause 1", Error::wrapr(TestError { msg: "cause 1".to_string(), inner: None }, "foo").ext().to_string());
        assert_eq!("cause 3", Error::wrapr(err, "foo").last().to_string());
    }

    #[test]
    fn test_assist_methods() {
        initialize();
        assert!(Error::raw("").is::<Error>());
        assert!(Error::raw("").downcast_ref::<Error>().is_some());
        assert!(Error::raw("").downcast_mut::<Error>().is_some());
    }
}
