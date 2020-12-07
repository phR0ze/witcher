use crate::error::Error;

/// `Result<T>` is a simplified return type to use throughout your application.
pub type Result<T> = std::result::Result<T, Error>;

pub trait ResultExt {

    /// Simply consumes the result ignoring it
    fn omit(&self);
}

impl ResultExt for std::fmt::Result {
    fn omit(&self) {
        let _ = match self {
            Ok(_) => (),
            Err(_) => (),
        };
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Write;
    
    #[test]
    fn test_the() {
        let mut w = String::new();
        write!(&mut w, "foobar").omit();
    }
}