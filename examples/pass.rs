use witcher::prelude::*;

fn do_something() -> Result<()> {
    do_external_thing().pass()
}

fn do_external_thing() -> std::io::Result<()> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "Oh no, we missed!"))?
}

fn main() {
    let err = do_something().unwrap_err();

    // Since we used `pass` we can match on the error directly
    match err.downcast_ref::<std::io::Error>() {
        Some(err) => println!("Root cause is std::io::Error: {}", err),
        None => println!("Root cause is witcher::Error: {}", err),
    }
}
