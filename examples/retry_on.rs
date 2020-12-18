use witcher::prelude::*;

fn retry_on_concreate_error_type() -> Result<()> {
    do_external_thing().retry_on(3, TypeId::of::<std::io::Error>(), |i| {
        println!("std::io::Error: retrying! #{}", i);
        do_external_thing()
    }).wrap("Failed while attacking beast")
}
fn do_external_thing() -> std::io::Result<()> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "Oh no, we missed!"))
}

fn main() {
    println!("{:?}", retry_on_concreate_error_type().unwrap_err());
}
