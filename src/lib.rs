mod backtrace;
mod error;
mod term;
mod wrapper;
use std::error::Error as StdError;

pub use crate::term::WITCHER_COLOR;
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
