// use crate::Error;
// use std::fmt::Display;

// /// Define the `wrap` function for Result types
// pub trait Wrapper<T, E> {

//     /// Wrap the error providing the ability to add more context
//     fn wrap<M>(self, msg: M) -> Result<T, Error>
//     where
//         M: Display + Send + Sync + 'static;
// }

// // Handle wrapping a std::error::Error
// impl<T, E> Wrapper<T, E> for Result<T, E>
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

// // Handle wrapping an Error
// // impl<T> Wrapper<T, Error> for Result<T, Error>
// // {
// //      fn wrap<M>(self, msg: M) -> Result<T, Error>
// //      where
// //          M: Display + Send + Sync + 'static,
// //      {
// //          //self.map_err(|err| Error::wrap(err, msg).unwrap_err())
// //      }
// // }

// // // Unit tests
// // // -------------------------------------------------------------------------------------------------
// // #[cfg(test)]
// // mod tests {
// //     use super::*;
// //     use std::fmt::Write;
    
// //     #[test]
// //     fn test_the() {
// //         let mut w = String::new();
// //         write!(&mut w, "foobar").omit();
// //     }
// // }