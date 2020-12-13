use std::any::Any;

pub trait Downcaster {
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub struct Wrapper {
    inner: Box<dyn Downcaster>,
}
impl Wrapper {
    pub fn new<D>(bar: D) -> Self
    where
        D: Downcaster + 'static
    {
        Self { inner: Box::new(bar) }
    }

    pub fn downcast_mut<D>(&mut self) -> Option<&mut D>
    where
        D: Downcaster + 'static
    {
        self.inner.as_any_mut().downcast_mut::<D>()
    }
}

pub struct Item {
    data: String,
}
impl Downcaster for Item {
    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

fn main() {
    let mut wrapper = Wrapper::new(Item{data: "foo".to_string()});
    println!("{}", wrapper.downcast_mut::<Item>().unwrap().data);
}