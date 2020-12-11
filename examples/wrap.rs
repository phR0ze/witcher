use witcher::prelude::*;

// Add additional context
fn do_something() -> Result<()> {
    do_another_thing().wrap("3rd wrap")
}

// Wrap a custom internal error from the get go
fn do_another_thing() -> Result<()> {
    do_final_thing().wrap("2nd wrap")
}

// Chain the external error using std::error::Error features
fn do_final_thing() -> Result<()> {
    let err = std::io::Error::new(std::io::ErrorKind::Other, "root cause!");
    Error::wrap(std::io::Error::new(std::io::ErrorKind::InvalidData, err), "1st wrap")
}

fn main() {
    println!("{}", do_something().unwrap_err());
}