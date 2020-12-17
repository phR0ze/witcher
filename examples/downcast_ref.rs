use witcher::prelude::*;

// Wrap our internal error with additional context as we move up the stack
fn do_something() -> Result<()> {
    do_external_thing().wrap("Failed to slay beast")
}

// Function that returns an external error type outside our codebase
fn do_external_thing() -> std::io::Result<()> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "Oh no, we missed!"))?
}

fn main() {
    let err = do_something().unwrap_err();

    // Get the last error in the error chain which will be the root cause
    let root_cause = err.last();

    // Match single concrete error type
    if let Some(e) = root_cause.downcast_ref::<std::io::Error>() {
        println!("Root cause is a std::io::Error: {}", e)
    } else {
        println!("{}", err)
    }
}
