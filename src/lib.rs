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
    pub use super::Result;
    pub use super::Error;
    pub use super::Wrapper;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
