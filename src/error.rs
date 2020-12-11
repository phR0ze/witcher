// use colored::*;
// use std::any::TypeId;
// use std::convert::From;
// use std::fmt::{self, Debug, Display, Formatter};
// use crate::term;
// use crate::Result;

// static ERROR_TYPE: &str = "witcher::Error";
// static LONG_ERROR_TYPE: &str = "witcher::error::Error";
// static SIMPLE_ERROR_TYPE: &str = "witcher::error::SimpleError";

// /// `Error` is a wrapper around lower level error types to provide additional context.
// /// 
// /// `Error` provides the following benefits
// ///  - ensures a backtrace will be taken at the earliest opportunity
// ///  - ensures that the error type is threadsafe and has a static lifetime
// ///  - provides matching on error types
// /// 
// /// Context comes in two forms. First every time an error is wrapped you have the
// /// opportunity to add an additional message. Finally a simplified stack trace is
// /// automatically provided that narrows in on your actual code ignoring the wind up
// /// and wind down that resides in the Rust std libraries and other dependencies
// /// allowing you to focus on your code.
// pub struct Error {

//     // Error messages
//     msg: String,

//     // // Original error type and name
//     // type_id: TypeId,
//     // type_name: String,

//     // Backtrace for the error
//     backtrace: Vec<crate::backtrace::Frame>,

//     // Wrapped error
//     // Wrapper<()> is a zero sized type (ZST), in this context, acting as a place holder for an
//     // unmanaged concrete instance of Wrapper. This means memory will need to be manually managed.
//     // wrapper: Box<Wrapper<()>>,
// }
// impl Error {

//     /// Create a new error instance using generics.
//     /// 
//     /// Supports any type that implements the trait bounds
//     pub fn new<M>(msg: M) -> Result<()>
//     where 
//         M: Display + Debug + Send + Sync + 'static
//     {
//         Err(Error {
//             msg: format!("{}", msg),
//             // type_id: TypeId::of::<Error>(),
//             // type_name: String::from(ERROR_TYPE),
//             backtrace: crate::backtrace::new(),
//         })
//     }

//     /// Wrap the given error and include a contextual message for the error.
//     ///
//     /// We require bounding with Send, Sync and 'static to ensure that the low level type
//     /// manipulation being done internally will be as safe as possible.
//     pub fn wrap<E, M>(err: E, msg: M) -> Result<()>
//     where
//         E: std::error::Error + Send + Sync + 'static,
//         M: Display + Send + Sync + 'static,
//     {
//         Err(Error {
//             msg: format!("{}", msg),
//             // type_id: TypeId::of::<Error>(),
//             // type_name: Error::name(&err),
//             backtrace: crate::backtrace::new(),
//         })
//         // let mut backtrace = None;
//         // let type_id = TypeId::of::<E>();

//         // // Ensure that we have a backtrace
//         // if type_id != TypeId::of::<Error>() {
//         //     backtrace = Some(crate::backtrace::new());
//         // }

//         // // Construct a public facing general error encapsulating all this detail
//         // Err(Error {
//         //     msg: format!("{}", msg),
//         //     // err_msg: format!("{}", err),

//         //     // Store the original error's type and name
//         //     // type_id: TypeId::of::<T>(),
//         //     type_name: Error::name(&err),
//         //     backtrace: backtrace,

//         //     // Construct a wrapper around the error's raw components for internal access
//         //     // wrapper: unsafe {
//         //     //     // Deconstruct the error Trait DST into is various raw constituent parts
//         //     //     // https://doc.rust-lang.org/src/core/raw.rs.html#60
//         //     //     let obj: TraitObject =  std::mem::transmute(&err as &dyn std::error::Error);
//         //     //     let wrapper = Wrapper {
//         //     //         vtable: obj.vtable,
//         //     //         error: err,
//         //     //     };

//         //     //     // Transmute the wrapper to ensure it is unmanaged and strip off its typing.
//         //     //     // This essentially eats the Box and removes any ownership from wrapper which
//         //     //     // means it will be a memory leak if we don't handle it manually.
//         //     //     std::mem::transmute(Box::new(wrapper))
//         //     // }
//         // })
//     }

//     /// Extract the name of the given error type and perform some clean up on the type
//     fn name<T>(_: T) -> String {
//         let mut name = format!("{}", std::any::type_name::<T>());

//         // Strip off prefixes
//         if name.starts_with('&') {
//             name = String::from(name.trim_start_matches('&'));
//         }

//         // Strip off suffixes
//         name = String::from(name.split("<").next().unwrap_or("<unknown>"));

//         // Hide SimpleError and full Error path
//         if name == SIMPLE_ERROR_TYPE {
//             name = String::from(ERROR_TYPE);
//         } else if name == LONG_ERROR_TYPE {
//             name = String::from(ERROR_TYPE);
//         }

//         name
//     }

//     // Common implementation for displaying error.
//     // A lifetime needs called out here for the frames and the frame references
//     // to reassure Rust that they will exist long enough to get the data needed.
//     fn fmt<'a, T>(&self, f: &mut Formatter<'_>, frames: T) -> fmt::Result
//     where 
//         T: Iterator<Item = &'a crate::backtrace::Frame>,
//     {
//         // Print out the error type and message
//         if self.msg != "" {
//             write!(f, " error: ")?;
//             if term::isatty() {
//                 writeln!(f, "{}: {}", ERROR_TYPE.bright_red(), self.msg.bright_red())?;
//             } else {
//                 writeln!(f, "{}: {}", ERROR_TYPE, self.msg)?;
//             }
//         }

//         // // Print out the wrapped error type and message
//         // write!(f, " error: ")?;
//         // if term::isatty() {
//         //     write!(f, "{}: ", self.wrapped.type_name.bright_red())?;
//         //     writeln!(f, "{}", format!("{:?}", self.wrapped.err).bright_red())?;
//         // } else {
//         //     writeln!(f, "{}: {:?}", self.wrapped.type_name, self.wrapped.err)?;
//         // }

//         // Print out the backtrace frames
//         for frame in frames {

//             // Add the symbol and file information
//             write!(f, "symbol: ")?;
//             if term::isatty() {
//                 writeln!(f, "{}", frame.symbol.bright_cyan())?;
//             } else {
//                 writeln!(f, "{}", frame.symbol)?;
//             }
//             write!(f, "    at: {}", frame.filename)?;

//             // Add the line and columen if they exist
//             if let Some(line) = frame.lineno {
//                 write!(f, ":{}", line)?;
//                 if let Some(column) = frame.column {
//                     write!(f, ":{}", column)?;
//                 }
//             }
//             write!(f, "\n")?;
//         }
//         Ok(())
//     }
// }

// // External trait implementations
// // -------------------------------------------------------------------------------------------------
// impl std::error::Error for Error {
// }

// /// Provides the same formatting for output as Display but includes the fullstack trace.
// impl Debug for Error {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         self.fmt(f, self.backtrace.iter())
//     }
// }

// /// Provides formatting for output with frames filtered to just target code
// impl Display for Error {
//     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
//         self.fmt(f, self.backtrace.iter().filter(|x| !x.is_dependency()))
//     }
// }

// // /// Provides conversion from std::error::Error into an Error
// // impl<E> From<E> for Error
// // where
// //     E: std::error::Error + Send + Sync + 'static,
// // {
// //     fn from(err: E) -> Error {
// //         Error::wrap(err, "").unwrap_err()
// //     }
// // }

// // Following the std::raw::TraitObject pattern rather than using it since it is nightly-only.
// // https://doc.rust-lang.org/std/raw/struct.TraitObject.html
// // https://doc.rust-lang.org/src/core/raw.rs.html
// //
// // This struct has the same internal layout as Trait DSTs e.g. &dyn Wrapper and Box<dyn Wrapper>
// // allowing you to deconstruct a Trait DST into its raw constituent parts that will no longer be
// // managed by the Rust memory management system and needs to be handled manually.
// //
// // TraitObject is guaranteed to match layouts by using the alternate repr(C) data representation to
// // get a reliable layout and thus can be used as targets for transmutes in unsafe code for
// // manipulating the raw representations directly. The only way to create values of this type is with
// // functions like std::mem::transmute. Similarly, the only way to create a true trait object from a
// // TraitObject value is with transmute.
// //
// // *const and *mut are equivalent in this context. I'm using *const to indicate that no change
// // is going to occur to the error's raw constituent parts.
// // #[repr(C)]
// // struct TraitObject {
// //     data: *const (),
// //     vtable: *const (),
// // }

// // Wrap an error's raw constituent parts with this wrapper so that we can easily refer to them
// // individually and manage memory manually while still being able to convert back into the
// // original typed error when needed. This will allow for managing any error type that implements
// // the trait std::error::Error and for complex operations like matching on error types.
// // Using alternate repr(C) data representation to get a reliable layout.
// // -------------------------------------------------------------------------------------------------
// // #[repr(C)]
// // struct Wrapper<T> {
// //     vtable: *const (),      // the original error's virtual method table
// //     error: T,               // the original error
// // }
// // impl Wrapper<()> {
// //     // Re-construct the original error from its raw constituent parts
// //     fn unwrap(&self) -> &(dyn std::error::Error + Send + Sync + 'static) {
// //         unsafe {
// //             // https://doc.rust-lang.org/src/core/raw.rs.html#69
// //             std::mem::transmute(TraitObject {
// //                 data: &self.error,      // gets coerced into *const type
// //                 vtable: self.vtable,
// //             })
// //         }
// //     }
// // }

// // Simple error is just an error with an un-named field for displaying a
// // simple message and keep the wrapping syntax clean and uniform.
// // -------------------------------------------------------------------------------------------------
// // struct SimpleError<M>(M)
// // where
// //     M: Display + Debug;

// // impl<M> std::error::Error for SimpleError<M>
// // where
// //     M: Display + Debug + Send + Sync + 'static {}

// // impl<M> Debug for SimpleError<M>
// // where
// //     M: Display + Debug,
// // {
// //     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
// //         Debug::fmt(&self.0, f)
// //     }
// // }

// // impl<M> Display for SimpleError<M>
// // where
// //     M: Display + Debug,
// // {
// //     fn fmt(&self, f: &mut Formatter) -> fmt::Result {
// //         Display::fmt(&self.0, f)
// //     }
// // }

// // // Unit tests
// // // -------------------------------------------------------------------------------------------------
// // #[cfg(test)]
// // mod tests {
// //     use super::*;
    
// //     fn io_error() -> crate::Result<()> {
// //         Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))?
// //     }

// //     #[test]
// //     fn test_new() {
// //         assert_eq!(String::from("foo"), Error::new("foo").msg);
// //         //assert_eq!(String::from("foo"), Error::new(String::from("foo")).msg);
// //         //assert_eq!(String::from("foo"), Error::new(Path::new("foo").display()).msg);
// //     }

// //     #[test]
// //     fn test_conversion_from_io_error() {
// //         let err = io_error().unwrap_err();
// //         // if let Some(e) = err.downcast_ref::<std::io::Error>() {
            
// //         // }
// //         assert_eq!("Custom { kind: Other, error: \"oh no!\" }", err.msg);
// //         //assert_eq!(err.msg, format!("{:?}", err.wrapped.unwrap()));
// //         //println!("Failed: {}", err);
// //     }
// // }