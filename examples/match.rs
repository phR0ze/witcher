// Import the essentials of error handling with a single line
use witcher::prelude::*;

// Wrap our internal error with additional context as we move up the stack
fn do_something() -> Result<()> {
    retry_on_concreate_error_type().wrap("Failed to slay beast")
}

fn retry_on_concreate_error_type() -> Result<()> {

    // Retry on concrete error using `err_is`
    let mut retries = 0;
    let mut result = do_external_thing();
    while retries < 3 && result.err_is::<std::io::Error>() {
        retries += 1;
        println!("std::io::Error: retrying using err_is #{}", retries);
        result = do_external_thing();
    }
    result.wrap("Failed while attacking beast")

    do_external_thing().retry_on(3, TypeId::of::<std::io::Error>(), |i| {
        println!("std::io::Error: retrying! #{}", i);
        do_external_thing()
    }).wrap("Failed while attacking beast")
}

// Function that returns an external error type outside our codebase
fn do_external_thing() -> std::io::Result<()> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "Oh no, we missed!"))
}

fn main() {
    let err = do_something().unwrap_err();

    // Get the last error in the error chain which will be the root cause
    let root_cause = err.last();

    // Match multiple cases to handle error differently based on first error
    match_err!(root_cause, {
        x: Error => println!("Last is witcher::Error: {}", x),
        x: std::io::Error => println!("Last is std::io::Error: {}", x),
        _ => println!("{}", root_cause)
    });
}
