use crate::{Error, Result, StdError};
use std::fmt::Display;

/// Define the `wrap` function for Result types
pub trait Wrapper<T> {

    /// Wrap the error providing the ability to add more context
    fn wrap<M>(self, msg: M) -> Result<T>
    where
        M: Display + Send + Sync + 'static;
}

// Handle wrapping a StdError
impl<T, E> Wrapper<T> for Result<T, E>
where 
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