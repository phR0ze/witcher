use witcher::prelude::*;

fn retry_on_concreate_error_type_using_err_is() -> Result<()> {
    let mut retries = 0;
    let mut result = do_external_thing();
    while retries < 3 && result.err_is::<std::io::Error>() {
        retries += 1;
        println!("retrying using err_is #{}", retries);
        result = do_external_thing();
    }
    result.wrap("Failed while attacking beast")
}
fn do_external_thing() -> std::io::Result<()> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "Oh no, we missed!"))
}

fn main() {
    println!("{}", retry_on_concreate_error_type_using_err_is().unwrap_err());
}
