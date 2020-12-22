use witcher::prelude::*;

// Add additional context
fn do_something() -> Result<()> {
    do_another_thing().wrap("2rd wrap")
}

// Add additional context
fn do_another_thing() -> Result<()> {
    do_external_thing().map_err(|err| {
        let e = err.downcast::<std::io::Error>().unwrap();
        Error::wrapr(*e, "1st wrap")
    })
}

// Chain the external error using std::error::Error features
// https://doc.rust-lang.org/rust-by-example/error/multiple_error_types/boxing_errors.html
fn do_external_thing() -> Result<(), Box<dyn std::error::Error>> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))?
}

fn main() {
    println!("{:?}", do_something().unwrap_err());
}
