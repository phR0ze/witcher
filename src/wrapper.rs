use crate::{Error, Result, StdError};
use std::any::TypeId;
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
            Error::wrap_err(err, msg).unwrap_err()
            // if TypeId::of::<E>() == TypeId::of::<Error>() {
            //     // Error::wrap(err, msg).unwrap_err()
            // } else {
            //     Error::wrap_err(err, msg).unwrap_err()
            // }
        })
    }
}

// // Box<(dyn StdError + Send + Sync + 'static)>
// impl<T> Wrapper<T> for std::result::Result<T, Box<(dyn StdError + Send + Sync + 'static)>>
// {
//     fn wrap<M>(self, msg: M) -> crate::Result<T>
//     where
//         M: Display + Send + Sync + 'static,
//     {
//         //self.map_err(|err| Error::wrap(err, msg).unwrap_err())
//         Error::new("test")
//     }
// }

// // Handle wrapping a Box<(dyn StdError + 'static)>
// impl<T> Wrapper<T> for Result<T, Box<dyn StdError + 'static>>
// where 
//     E: StdError + Send + Sync + 'static,
// {
//     fn wrap<M>(self, msg: M) -> Result<T, Error>
//     where
//         M: Display + Send + Sync + 'static,
//     {
//         self.map_err(|err| Error::wrap(err, msg).unwrap_err())
//     }
// }

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