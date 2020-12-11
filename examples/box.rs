use witcher::prelude::*;
use std::{io,result,error};

// Add additional context
fn do_something() -> Result<()> {
    do_another_thing().wrap("2rd wrap")
}

// Add additional context
fn do_another_thing() -> Result<()> {
    //do_final_thing().wrap("2nd wrap")
    match do_final_thing() {
        Err(e) => {
            Error::wrap_box(e, "1st wrap")
        },
        Ok(_) => Ok(()),
    }
}

// Chain the external error using std::error::Error features
fn do_final_thing() -> result::Result<(), Box<dyn error::Error + Send + Sync + 'static>> {
    Err(io::Error::new(io::ErrorKind::Other, "root cause!"))?
}

fn main() {
    println!("{}", do_something().unwrap_err());
}