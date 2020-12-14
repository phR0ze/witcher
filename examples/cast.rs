trait Foo {
    fn foo(&self) -> String;
}

struct Bar(String);
impl Foo for Bar {
    fn foo(&self) -> String {
        format!("foo: {}", &self.0)
    }
}

fn main() {
    let mut foos: Vec<&dyn Foo> = Vec::new();

    // Coerce concrete type Bar into trait type Foo
    let foo1: &dyn Foo = &Bar("bar1".to_string());
    foos.push(foo1);

    // Cast concrete type Bar into trait type Foo
    let foo2 = &Bar("bar2".to_string()) as &dyn Foo;
    foos.push(foo2);

    // Reverse and consume foos
    for foo in foos.into_iter().rev() {
        println!("{}", foo.foo());
    }
}