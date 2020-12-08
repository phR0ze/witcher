use witcher::prelude::*;

fn do_something() -> Result<()> {
    do_another_something()
}

fn do_another_something() -> Result<()> {
    Err(Error::new("oh no!"))
}

fn main() {
    println!("{}", do_something().unwrap_err());
}