pub mod error;
pub mod term;
pub mod backtrace;
pub mod result;

/// Import all essential symbols in a simple consumable way
///
/// ### Examples
/// ```
/// use witcher::prelude::*;
/// ```
pub mod prelude {
    pub use super::result::*;
    pub use super::error::Error;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
