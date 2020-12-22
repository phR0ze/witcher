mod backtrace;
mod error;
mod term;
mod wrapper;
use std::error::Error as StdError;

/// Environment variable name for enabling/disabling color
pub const WITCHER_COLOR: &str = "WITCHER_COLOR";

/// Environment variable name for enabling/disabling fullstack tracing
pub const WITCHER_FULLSTACK: &str = "WITCHER_FULLSTACK";

pub use crate::error::Error;
pub use crate::wrapper::Wrapper;

/// `Result<T>` is a simplified return type to use throughout your application.
pub type Result<T, E = Error> = std::result::Result<T, E>;

/// Import all essential symbols in a simple consumable way
///
/// ### Examples
/// ```
/// use witcher::prelude::*;
/// ```
pub mod prelude {
    pub use super::WITCHER_COLOR;
    pub use super::WITCHER_FULLSTACK;
    pub use super::bail;
    pub use super::err;
    pub use super::wrap;
    pub use super::match_err;
    pub use super::Result;
    pub use super::Error;
    pub use super::Wrapper;
    pub use std::any::TypeId;
}

/// Bail early from a function with an `Error`.
/// 
/// `bail!` just provides an implementation of the common error handling practice of allowing
/// a user to return immediately with an error. Using `bail!("oh no!")` is the same thing as
/// if you were to use `return Error::new("oh no!")` or `return Err(Error::raw("oh no!")`.
/// 
/// It also provides a variation to allow for format!() type formatting.
/// 
/// ### Examples
/// ```rust,ignore
/// bail!("oh no!");
/// bail!("foo: {}", "oh no!");
/// ```
#[macro_export]
macro_rules! bail {
    // Simple message
    ($msg:expr) => {
        return $crate::Error::new($msg);
    };

    // format! style formatting
    ($fmt:expr, $($arg:tt)*) => {
        return $crate::Error::new(&format!($fmt, $($arg)*));
    };
}

/// `err!` works just like `bail!` but doesn't return
/// 
/// just a simple way to get string formatting like `format!` for new errors.
/// The same could be done with `Error::raw(format!("{}", msg))` but is more verbose.
/// 
/// ### Examples
/// ```rust,ignore
/// err!("oh no!");
/// err!("foo: {}", "oh no!");
/// ```
#[macro_export]
macro_rules! err {
    // Simple message
    ($msg:expr) => {
        $crate::Error::raw($msg);
    };

    // format! style formatting
    ($fmt:expr, $($arg:tt)*) => {
        $crate::Error::raw(&format!($fmt, $($arg)*));
    };
}

/// `wrap!` behaves much like the venerable `bail!` but wraps an external error
/// 
/// It also provides a variation to allow for format!() type formatting.
/// 
/// ### Examples
/// ```rust,ignore
/// wrap!(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"), "wrapper msg");
/// ```
#[macro_export]
macro_rules! wrap {
    // Simple message
    ($err:expr, $msg:expr) => {
        return $crate::Error::wrap($err, $msg);
    };

    // format! style formatting
    ($err:expr, $fmt:expr, $($arg:tt)*) => {
        return $crate::Error::wrap($err, &format!($fmt, $($arg)*));
    };
}

/// Match on error types.
/// This only works with errors implementing the `std::error::Error` trait as it makes use of
/// the standard `is` and `downcast_ref` implementations.
/// 
/// ### Examples
/// ```rust
/// use witcher::prelude::*;
/// let err = std::io::Error::new(std::io::ErrorKind::Other, "oh no!");
/// let res = match_err!(&err, {
///     _x: std::io::Error => true,
///     _ => false
/// });
/// assert!(res);
/// ```
#[macro_export]
macro_rules! match_err {
    ($err:expr, { $($var:ident : $kind:ty => $arm:expr),*, _ => $default:expr }) => (
        $(
            if ($err as &(dyn std::error::Error + 'static)).is::<$kind>() {
                let $var = ($err as &(dyn std::error::Error + 'static)).downcast_ref::<$kind>().unwrap();
                $arm
            } else
        )*
        {
            $default
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{env, fmt, io};

    // Disable backtrace and colors
    use std::sync::Once;
    static INIT: Once = Once::new();
    pub fn initialize() {
        INIT.call_once(|| {
            env::set_var(crate::WITCHER_COLOR, "0");
            env::set_var("RUST_BACKTRACE", "0");
        });
    }

    #[derive(Debug)]
    struct TestError1(String);
    impl std::error::Error for TestError1 {}
    impl fmt::Display for TestError1 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Debug)]
    struct TestError2(String);
    impl std::error::Error for TestError2 {}
    impl fmt::Display for TestError2 {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    fn bail_simple() -> Result<()> {
        bail!("oh no!");
    }

     fn bail_formatted() -> Result<()> {
        bail!("foo: {}", "oh no!");
    }
   
    fn wrap_simple() -> Result<()> {
        wrap!(io::Error::new(io::ErrorKind::NotFound, "oh no!"), "simple_wrap");
    }

     fn wrap_formatted() -> Result<()> {
        wrap!(io::Error::new(io::ErrorKind::NotFound, "oh no!"), "foo: {}", "simple_wrap");
    }
     
    #[test]
    fn test_bail() {
        initialize();
        assert_eq!("oh no!", bail_simple().unwrap_err().to_string());
        assert_eq!("foo: oh no!", bail_formatted().unwrap_err().to_string());
    } 
 
    #[test]
    fn test_err() {
        initialize();
        assert_eq!("oh no!", err!("oh no!").to_string());
        assert_eq!("foo: oh no!", err!("foo: {}", "oh no!").to_string());
    } 
   
    #[test]
    fn test_wrap() {
        initialize();
        assert_eq!("simple_wrap", format!("{}", wrap_simple().unwrap_err()));
        assert_eq!(" error: simple_wrap\n cause: oh no!", format!("{:#}", wrap_simple().unwrap_err()));
        assert_eq!("foo: simple_wrap", wrap_formatted().unwrap_err().to_string());
        assert_eq!(" error: foo: simple_wrap\n cause: oh no!", format!("{:#}", wrap_formatted().unwrap_err()));
    } 
    
    #[test]
    fn test_single() {
        initialize();
        let err = io::Error::new(std::io::ErrorKind::Other, "oh no!");
        let res = match_err!(&err, {
            _x: io::Error => true,
            _ => false
        });
        assert!(res);
    } 
  
    #[test]
    fn test_match_err() {
        initialize();
        let errors: Vec<Box<dyn std::error::Error>> = vec![
            Box::new(TestError1("test1".to_string())),
            Box::new(TestError2("test2".to_string())),
            Box::new(io::Error::new(std::io::ErrorKind::Other, "test3")),
        ];

        let mut buf = String::new();
        for boxed in errors.iter() {
            let err: &(dyn StdError + 'static) = &**boxed;
            buf += & match_err!(err, {
                x: io::Error => format!("io::Error: {}\n", x),
                x: TestError1 => format!("TestError1: {}\n", x),
                x: TestError2 => format!("TestError2: {}\n", x),
                _ => format!("no match")
            });
        }
        assert_eq!("TestError1: test1\nTestError2: test2\nio::Error: test3\n", buf);
    } 
}
