pub mod error;
pub mod term;
pub mod backtrace;
pub mod wrapper;

/// `Result<T>` is a simplified return type to use throughout your application.
pub type Result<T, E = error::Error> = std::result::Result<T, E>;

/// Import all essential symbols in a simple consumable way
///
/// ### Examples
/// ```
/// use witcher::prelude::*;
/// ```
pub mod prelude {
    pub use super::Result;
    pub use super::error::Error;
    pub use super::wrapper::Wrapper;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
