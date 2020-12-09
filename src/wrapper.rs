use crate::error::Error;
use crate::Result;

/// Define the `wrap` function for Result types
pub trait Wrapper<T, E> {

    /// Wrap the error with additional context
    fn wrap<M>(self, msg: M) -> Result<T, Error>
    where
        M: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static;
}

impl<T, E> Wrapper<T, E> for Result<T, E>
where 
    E: std::error::Error + Send + Sync + 'static,
{
    fn wrap<M>(self, msg: M) -> Result<T>
    where
        M: std::fmt::Display + std::fmt::Debug + Send + Sync + 'static,
    {
        self.map_err(|err| Error::wrap(err, msg))
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