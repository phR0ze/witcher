use crate::Error;
use crate::Result;
use std::fmt::Display;

/// Define the `wrap` function for Result types
pub trait Wrapper<T> {

    /// Wrap the error providing the ability to add more context
    fn wrap<M>(self, msg: M) -> Result<T>
    where
        M: Display + Send + Sync + 'static;
}

// Handle wrapping a std::error::Error
impl<T, E> Wrapper<T> for Result<T, E>
where 
    E: std::error::Error + Send + Sync + 'static,
{
    fn wrap<M>(self, msg: M) -> Result<T>
    where
        M: Display + Send + Sync + 'static,
    {
        self.map_err(|err| Error::wrap(err, msg).unwrap_err())
    }
}

// // Box<(dyn std::error::Error + Send + Sync + 'static)>
// impl<T> Wrapper<T> for std::result::Result<T, Box<(dyn std::error::Error + Send + Sync + 'static)>>
// {
//     fn wrap<M>(self, msg: M) -> crate::Result<T>
//     where
//         M: Display + Send + Sync + 'static,
//     {
//         //self.map_err(|err| Error::wrap(err, msg).unwrap_err())
//         Error::new("test")
//     }
// }

// // Handle wrapping a Box<(dyn std::error::Error + 'static)>
// impl<T> Wrapper<T> for Result<T, Box<dyn std::error::Error + 'static>>
// where 
//     E: std::error::Error + Send + Sync + 'static,
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