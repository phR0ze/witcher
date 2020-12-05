use witcher::prelude::*;

fn do_something() -> Result<()> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))?
}

fn main() {
    println!("{}", do_something().unwrap_err());
}
