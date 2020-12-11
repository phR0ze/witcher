// Documenting the Wrapper concept design prototype here
// ---------------------------------------------------------------------------------------
use std::fmt::{self, Display};

// Implement result wrapper
// ---------------------------------------------------------------------------------------
type Result<T, E = Error> = std::result::Result<T, E>;

trait Wrapper<T, E> {
    fn wrap<M>(self, msg: M) -> Result<T>
    where
        M: Display + Send + Sync + 'static;
}

impl<T, E> Wrapper<T, E> for Result<T, E>
where
    E: StdErr + Send + Sync + 'static,
{
    fn wrap<M>(self, msg: M) -> Result<T, Error>
    where
        M: Display + Send + Sync + 'static,
    {
        self.map_err(|err| Error::wrap(err, msg).unwrap_err())
    }
}

// Implement std::error::Error mock
// ---------------------------------------------------------------------------------------
trait StdErr: Display {
    fn inner(&self) -> Option<&(dyn StdErr + Send + Sync + 'static)>;
}

struct CircleErr;
impl StdErr for CircleErr {
    fn inner(&self) -> Option<&(dyn StdErr + Send + Sync + 'static)> { None }
}
impl Display for CircleErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "circle")
    }
}

struct SquareErr;
impl StdErr for SquareErr {
    fn inner(&self) -> Option<&(dyn StdErr + Send + Sync + 'static)> { None }
}
impl Display for SquareErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "square")
    }
}

struct SquircleErr {
    inner: Option<Box<dyn StdErr + Send + Sync + 'static>>,
}
impl StdErr for SquircleErr {
    fn inner(&self) -> Option<&(dyn StdErr + Send + Sync + 'static)> {
        match &self.inner {
            Some(x) => Some(&(**x)),
            None => None,
        }
    }
}
impl Display for SquircleErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "squircle")
    }
}

// Implement the wrapper error
// ---------------------------------------------------------------------------------------
struct Error {
    msg: String,
    inner: Option<Box<dyn StdErr + Send + Sync + 'static>>,
}
impl Error {
    fn new<M>(msg: M) -> Result<()>
    where
        M: Display + Send + Sync + 'static,
    {
        Err(Error {
            msg: format!("{}", msg),
            inner: None,
        })
    }

    fn wrap<S, M>(shape: S, msg: M) -> Result<()>
    where
        S: StdErr + Send + Sync + 'static,
        M: Display + Send + Sync + 'static,
    {
        Err(Error {
            msg: format!("{}", msg),
            inner: Some(Box::new(shape)),
        })
    }

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        writeln!(f, "{}", self.msg)?;
        if let Some(inner) = self.inner() {
            inner.fmt(f)?;
        }
        Ok(())
    }
}
impl StdErr for Error {
    fn inner(&self) -> Option<&(dyn StdErr + Send + Sync + 'static)> {
        match &self.inner {
            Some(x) => Some(&(**x)),
            None => None,
        }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.fmt(f)
    }
}


// Test design
// ---------------------------------------------------------------------------------------

fn std_do_something() -> Result<()> {
    std_do_another_thing().wrap("2nd wrap")
}

fn std_do_another_thing() -> Result<()> {
    std_do_final_thing().wrap("1st wrap")
}

fn std_do_final_thing() -> Result<(), SquircleErr> {
    Err(SquircleErr{inner: Some(Box::new(CircleErr))})
}

fn do_something() -> Result<()> {
    do_another_thing().wrap("2nd wrap")
}

fn do_another_thing() -> Result<()> {
    do_final_thing().wrap("1st wrap")
}

fn do_final_thing() -> Result<()> {
    Error::new("root")
}

fn main() {
    println!("Test StdErr origin");
    println!("{}", std_do_something().unwrap_err());

    println!("\nTest Error origin");
    println!("{}", do_something().unwrap_err());
}