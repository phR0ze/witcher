use witcher::prelude::*;

fn retry() -> Result<()> {
    do_external_thing().retry(3, |i| {
        println!("std::io::Error: retrying! #{}", i);
        do_external_thing()
    }).wrap("Failed while attacking beast")
}
fn do_external_thing() -> std::io::Result<()> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "Oh no, we missed!"))
}

fn main() {
    println!("{:?}", retry().unwrap_err());
}
