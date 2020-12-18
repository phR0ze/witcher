use witcher::prelude::*;
#[derive(Debug)]
struct SuperError {
    side: SuperErrorSideKick,
}
impl std::fmt::Display for SuperError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SuperError is here!")
    }
}
impl std::error::Error for SuperError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.side)
    }
}

#[derive(Debug)]
struct SuperErrorSideKick;
impl std::fmt::Display for SuperErrorSideKick {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SuperErrorSideKick is here!")
    }
}
impl std::error::Error for SuperErrorSideKick {}

fn do_something() -> Result<()> {
    do_external_thing().wrap("Failed doing super hero work")
}

fn do_external_thing() -> std::result::Result<(), SuperError> {
    Err(SuperError {side: SuperErrorSideKick})
}

fn main() {
    if let Err(err) = do_something() {

        // Traverse the error chain
        let mut source = Some(err.std());
        while let Some(err) = source {
            match_err!(err, {
                // Using alternate form of display for `Error` to get just the message
                x: Error => println!("Found witcher::Error: {}", x),
                x: SuperError => println!("Found SuperError: {}", x),
                x: SuperErrorSideKick => println!("Found SuperErrorSideKick: {}", x),
                _ => println!("unknown")
            });
            source = err.source();
        }
    }
}