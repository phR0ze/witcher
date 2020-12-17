use crate::{Error, Result, StdError};
use std::fmt::{Debug, Display};

/// Define the `wrap` function for Result types
pub trait Wrapper<T> {

    /// Wrap the error providing the ability to add more context
    fn wrap<M>(self, msg: M) -> Result<T>
    where
        M: Display + Send + Sync + 'static;

    /// Check if there is an error and the err is the given error type
    fn err_is<U>(&self) -> bool
    where
        U: StdError + 'static;
    
    /// Execute the given function when we have an error `max` number of times.
    fn retry<U, F>(self, max: usize, f: F) -> Result<T>
    where 
        U: StdError + 'static,
        F: FnOnce(usize, U) -> Error;
}

// Handle wrapping a StdError
impl<T, E> Wrapper<T> for std::result::Result<T, E>
where 
    T: Debug + Send + Sync + 'static,
    E: StdError + Send + Sync + 'static,
{
    fn wrap<M>(self, msg: M) -> Result<T>
    where
        M: Display + Send + Sync + 'static,
    {
        self.map_err(|err| {
            Error::wrap(err, msg).unwrap_err()
        })
    }

    fn err_is<U>(&self) -> bool
    where
        U: StdError + 'static
    {
        match self {
            Ok(_) => false,
            Err(e) => (e as &(dyn StdError + 'static)).is::<U>(),
        }
    }

    fn retry<U, F>(self, max: usize, f: F) -> Result<T>
    where 
        U: StdError + 'static,
        F: FnOnce(usize, U) -> Error
    {
        let mut retries = 0;
        while retries < max && self.is_err() {
            retries += 1;
        }
        Err(Error::raw(""))
    }
}

// // Unit tests
// // -------------------------------------------------------------------------------------------------
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use std::fmt::Write;
    
//     #[test]
//     fn test_the() {
//         let mut w = String::new();
//         write!(&mut w, "foobar").omit();
//     }
// }