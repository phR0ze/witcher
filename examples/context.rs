use witcher::prelude::*;

fn do_first_thing() -> Result<()> {
    do_second_thing()
}

fn do_second_thing() -> Result<()> {
    do_third_thing().wrap("second context")
}

fn do_third_thing() -> Result<()> {
    Err(Error::new("oh no!"))?
}

fn main() {
    println!("{}", do_first_thing().unwrap_err());
}