use witcher::{Error, Result};

fn do_first_thing() -> Result<()> {
    println!("First thing");
    do_second_thing()
}

fn do_second_thing() -> Result<()> {
    println!("Second thing");
    do_third_thing()
}

fn do_third_thing() -> Result<()> {
    println!("Third thing");
    Err(Error::new("failed to do third thing"))?
}

fn main() {
    match do_first_thing() {
        Ok(_) => println!("did something successfully"),
        Err(e) => println!("Failed: {}", e),
    };
}