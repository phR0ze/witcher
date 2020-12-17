use crate::{Error, Result, StdError};
use std::fmt::{Debug, Display};

/// Define the `wrap` function for Result types
pub trait Wrapper<T> {

    /// Wrap the error providing the ability to add more context
    fn wrap<M>(self, msg: M) -> Result<T>
    where
        M: Display + Send + Sync + 'static;

    /// Check if the error is the given error type
    fn err_is<U: StdError + 'static>(&self) -> bool;
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

    fn err_is<U: StdError + 'static>(&self) -> bool
    {
        match self {
            Ok(_) => false,
            Err(e) => (e as &(dyn StdError + 'static)).is::<U>(),
        }
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