use witcher::prelude::*;

// Add additional context
fn do_something() -> Result<()>
{
    do_another_thing().wrap("1st wrap")
}

// Originate a new simple error
fn do_another_thing() -> Result<()>
{
    Error::new("oh no!")
}

fn main()
{
    println!("{:?}", do_something().unwrap_err());
}
