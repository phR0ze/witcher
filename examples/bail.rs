use witcher::prelude::*;

// Add additional context
fn do_something() -> Result<()> {
    bail!("failed")
}

fn main() {
    println!("{:?}", do_something().unwrap_err());
}
