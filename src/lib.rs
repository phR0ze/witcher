pub mod error;
pub mod term;
pub mod backtrace;
pub mod misc;

/// `Error` is a wrapper around lower level error types to provide additional context.
#[derive(Debug)]
pub struct Error {
    message: &'static str,
    backtrace: String,
}

/// `Result<T, Error>` is a simplified return type to use throughout your application.
pub type Result<T, E = Error> = std::result::Result<T, E>;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
