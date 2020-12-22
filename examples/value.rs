use witcher::prelude::*;

fn do_something() -> Result<String> {
    do_another_thing().wrap("1st wrap")
}

fn do_another_thing() -> Result<String> {
    Error::new("oh no!")
}

fn do_value_thing() -> Result<String> {
    Ok("return value".into())
}

fn main() {
    println!("{}", do_value_thing().unwrap());
    println!("{:?}", do_something().unwrap_err());
}
