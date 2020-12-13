use std::any::Any;
use std::fmt::Display;

pub trait Downcaster {
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct Error {
    msg: String,
    inner: Option<Box<dyn Downcaster + Send + Sync + 'static>>,
}
impl Error {
    pub fn new<M>(msg: M) -> Self
    where
        M: Display + Send + Sync + 'static
    {
        Self {
            msg: format!("{}", msg),
            inner: None,
        }
    }

    pub fn wrap<E, M>(err: E, msg: M) -> Self
    where
        E: Downcaster + Send + Sync + 'static,
        M: Display + Send + Sync + 'static
    {
        Self {
            msg: format!("{}", msg),
            inner: Some(Box::new(err)),
        }
    }

    pub fn downcast_mut<E>(&mut self) -> Option<&mut E>
    where
        E: Downcaster + 'static
    {
        match &mut self.inner {
            Some(inner) => inner.as_any_mut().downcast_mut::<E>(),
            None => None,
        }
    }
}
impl Downcaster for Error {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
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