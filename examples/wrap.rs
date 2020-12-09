use witcher::prelude::*;

fn do_first_thing() -> Result<()> {
    do_second_thing()
}

fn do_second_thing() -> Result<()> {
    do_third_thing().wrap("second wrap")
}

fn do_third_thing() -> Result<()> {
    Error::wrap(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"), "third")
}

fn main() {
    println!("{}", do_first_thing().unwrap_err());
}