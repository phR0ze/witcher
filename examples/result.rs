use witcher::prelude::*;

// Wrap an external error with additional context
fn do_something() -> Result<()> {
    do_another_thing().wrap("1st wrap")
}

// Create an external error to wrap
fn do_another_thing() -> std::io::Result<()> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))
}

fn main() {
    println!("{}", do_something().unwrap_err());
}
