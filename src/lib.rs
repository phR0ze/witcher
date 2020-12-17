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
    pub use super::match_err;
    pub use super::Result;
    pub use super::Error;
    pub use super::Wrapper;
}

/// Match on error types.
/// This only works with errors implementing the `std::error::Error` trait as it makes use of
/// the standard `is` and `downcast_ref` functions.
/// 
/// Ensure that the error your working with is a reference.
///
/// ### Examples
/// ```
/// use witcher::prelude::*;
/// let err = io::Error::new(std::io::ErrorKind::Other, "oh no!");
/// let res = match_err!(&err, {
///     _x: io::Error => true,
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
   
    #[test]
    fn test_single() {
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
