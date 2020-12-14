use std::any::Any;
use std::fmt::{Debug, Display};

trait Downcaster {
    fn downcast_mut<E>(&mut self) -> Option<&mut E>
    where
        E: Debug + Send + Sync + 'static;
}

#[derive(Debug)]
struct Error {
    msg: String,
    inner: Option<Box<dyn Any + Send + Sync + 'static>>,
}
impl Error {
    fn new<M>(msg: M) -> Self
    where
        M: Display + Send + Sync + 'static
    {
        Self {
            msg: format!("{}", msg),
            inner: None,
        }
    }

    fn wrap<E, M>(err: E, msg: M) -> Self
    where
        E: Debug + Send + Sync + 'static,
        M: Display + Send + Sync + 'static
    {
        Self {
            msg: format!("{}", msg),
            inner: Some(Box::new(err)),
        }
    }
}
impl Downcaster for Error {
    fn downcast_mut<E>(&mut self) -> Option<&mut E>
    where
        E: Debug + Send + Sync + 'static
    {
        match &mut self.inner {
            Some(inner) => inner.downcast_mut::<E>(),
            None => None,
        }
    }
}

fn main() {
    let mut wrapper = Error::wrap(Error::new("msg1"), "msg2");
    let err1 = wrapper.downcast_mut::<Error>().unwrap();
    println!("{}", err1.msg);
    err1.msg += " - modified it foo";
    println!("{}", err1.msg);
    println!("{}", wrapper.msg);
    let err2 = wrapper.downcast_mut::<Error>().unwrap();
    println!("{}", err2.msg);
}