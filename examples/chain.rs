use witcher::prelude::*;
use std::fmt::{self, Formatter, Display, Debug};
use std::error::Error as StdError;

struct TestError {
    msg: String,
    inner: Option<Box<TestError>>
}
impl Debug for TestError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
impl Display for TestError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{}", self.msg)
    }
}
impl StdError for TestError {
    fn source(&self) -> Option<&(dyn StdError + 'static)>
    {
        match &self.inner {
            Some(x) => Some(x as &dyn StdError),
            None => None,
        }
    }
}

// Add additional context
fn do_something() -> Result<()> {
    do_another_thing().wrap("3rd wrap")
}

// Wrap a custom internal error from the get go
fn do_another_thing() -> Result<()> {
    do_external_thing().wrap("2nd wrap")
}

// Chain the external error using std::error::Error features
fn do_external_thing() -> Result<()> {
    let err = TestError {
        msg: "cause 1".to_string(),
        inner: Some(Box::new(TestError{
            msg: "cause 2".to_string(),
            inner: Some(Box::new(TestError{
                msg: "cause 3".to_string(),
                inner: None
            })),
        })),
    };
    Error::wrap(err, "1st wrap")
}

fn main() {
    println!("{}", do_something().unwrap_err());
}