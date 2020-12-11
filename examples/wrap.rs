use witcher::prelude::*;

// Add additional context
fn do_something() -> Result<i32> {
    do_another_thing().wrap("2nd wrap")
}

// Wrap a custom internal error from the get go
fn do_another_thing() -> Result<i32> {
    do_final_thing().wrap::<i32>("1st wrap")
}

// Chain the external error using std::error::Error features
fn do_final_thing() -> std::io::Result<()> {
    let err = std::io::Error::new(std::io::ErrorKind::Other, "root cause!");
    Err(std::io::Error::new(std::io::ErrorKind::InvalidData, err))
}

fn main() {
    println!("{}", do_something().unwrap_err());
}