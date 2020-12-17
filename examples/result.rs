use witcher::prelude::*;

// Wrap an error with additional context
fn do_something() -> Result<()> {
    do_another_thing().wrap("Failed to slay beast")
}

// Wrap an external error with additional context
fn do_another_thing() -> Result<()> {
    do_final_thing().wrap("Failed during sword swing")
}

// Create an external error to wrap
fn do_final_thing() -> std::io::Result<()> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "Oh no, we missed!"))
}

fn main() {
    println!("{}", do_something().unwrap_err());
}
