use witcher::prelude::*;

// Wrap an error with additional context
fn do_something() -> Result<()> {
    do_another_thing().wrap("Failed to slay beast")
}

// Wrap an external error with additional context
fn do_another_thing() -> Result<()> {
    do_final_thing().wrap("Failed during sword swing")
}

// Create an external error to wrap
fn do_final_thing() -> std::io::Result<()> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "Oh no, we missed!"))
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    match args.get(1) {
        Some(arg) => match arg.as_str() {
            "normal" => println!("{}", do_something().unwrap_err()),
            "alternate" => println!("{:#}", do_something().unwrap_err()),
            "debug" => println!("{:?}", do_something().unwrap_err()),
            "alternate-debug" => println!("{:#?}", do_something().unwrap_err()),
            _ => println!("{:?}", do_something().unwrap_err()),
        },
        _ => println!("{:?}", do_something().unwrap_err()),
    }
}
