
use witcher::prelude::*;

fn main() {
    let err = Error::wrap(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"));
    println!("{}", err);
}
