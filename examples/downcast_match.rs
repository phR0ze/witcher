use witcher::prelude::*;

fn do_something() -> Result<()>
{
    do_external_thing().wrap("Failed to slay beast")
}

fn do_external_thing() -> std::io::Result<()>
{
    Err(std::io::Error::new(std::io::ErrorKind::Other, "Oh no, we missed!"))?
}

fn main()
{
    let err = do_something().unwrap_err();

    // Match multiple downcasted cases to handle errors differently
    match_err!(err.last(), {
        x: Error => println!("Root cause is witcher::Error: {}", x),
        x: std::io::Error => println!("Root cause is std::io::Error: {}", x),
        _ => println!("{:?}", err)
    });
}
