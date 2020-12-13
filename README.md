# witcher
Track and put down bugs using simple concise error handling

## Error Handling Tenants <a name="error-handling-tentants"/></a>
1. ***Error handling should be simple***  
   > by providing pattern matching on errors  
   > by automatically handling conversions and wrapping  
   > by providing concise and terse user interaction  
2. ***Error handling should tell the full story***  
   > by never discarding errors  
   > by chaining errors together  
   > by providing tracing from point of origin  

### Table of Contents
* [Usage](#usage)
  * [Return Result](#return-result)
  * [Add Additional Context](#add-additional-context)
* [Design](#design)
  * [Manifesto](#manifesto)
  * [Concepts](#concepts-implemented)
  * [Terminate](#terminate)
  * [Default value](#default-value)
  * [Coerce errors](#coerce-errors)
  * [Downcast errors](#downcast-errors)
* [Trait Objects](#trait-objects)
  * [Storing trait objects](#storing-trait-objects)
  * [Fat pointers](#fat-pointers)
  * [Dynamic Dispatch](#dynamic-dispatch)
    * [vtable](#vtable)
  * [Transmutting](#transmutting)
  * [Data layouts](#data-layouts)
    * [Size and Alignment](#size-and-alignment)
    * [repr(C)](#repr-c)
* [Error Packages](#error-packages)
  * [Error Handling](#error-handling)
    * [failure](#failure)
    * [error-chain](#error-chain)
    * [anyhow](#anyhow)
    * [tracing-error](#tracing-error)
  * [Error Generation](#error-generation)
    * [quick-error](#quick-error)
    * [error-type](#error-type)
    * [thiserror](#thiserror)

# Usage <a name="usage"/></a>

## Return Result <a name="return-result"/></a>
Rather than terminate execution randomly in your code base, with `panics` and/or `unwrap` and
`expect`, or laboriously deal with every error type, or constantly `Box` and unbox values simply use
`witcher::Result<T>` as the return type and the `?` operator to automatically propogate errors up the
stack simply and easily.

Example:
```rust
use witcher::prelude::*;

fn do_something() -> Result<()> {
    Err(std::io::Error::new(std::io::ErrorKind::Other, "oh no!"))?
}

fn main() {
    println!("{}", do_something().unwrap_err());
}
```
```console
Custom { kind: Other, error: "oh no!" }
result::do_something
  at examples/result.rs:4:65
result::main
  at examples/result.rs:8:19
```

## Add Additional Context <a name="add-additional-context"/></a>
Add additional context along the failure path to get a better understanding of what was being done
when a cryptic low level error is returned. This is done with the `wrap` method implemented for all
`std::error::Error` and all `std::result::Result`. You can either add an inline description or an
additional error for better high level programatic decisions.

Witcher supports 3 primary cases with contexts:
1. a simple message
2. a message using format like parameters
3. an error implementing the std::error::Error trait

Example:
```rust
use std::io;
use witcher::prelude::*;

fn do_something() -> Result<()> {
    io::Error::new(io::ErrorKind::Other, "oh no!").wrap("Failed to do_something important")?
}

fn main() {
    let err = do_something().unwrap_err();
    println!("{}", err);
}
```

# Design <a name="design"/></a>
From my perspective errors should capture enough meta data around a problem to give a developer
a good chance of finding the error's origin without having to try and reproduce the problem. This
means including meta data along the entire trace the execution thread took with developer specific
comments along the way. Finally error handling should be convenient to work with and have minimal
overhead or developers won't use them.

## Manifesto <a name="manifesto"/></a>
Coming from a Golang background most recently I wasn't that put off by Rust's slow improvements with
error handling. I fully expected I'd just import the defacto standard error package similar to
something like Golang's [pkg/errors](https://github.com/pkg/errors). Digging in I found a rich
anthropological layered history of projects with nobal ideals and principles. I found a few clearly
more used than others and saw the tide turn one once popular packages. Literally weeks of
research and testing of numerous different patterns and packages later though I have still yet to
find anything as solid and usable as the venerable [pkg/errors](https://github.com/pkg/errors). So
I'll add to this archaeological dig with my own take on error handling.

## Concepts <a name="concepts"/></a>

### Error <a name="error"/></a>
Most error handling implementations have the concept of a new error type `Error` that will be used to
wrap other errors and contain more information that is available with the the venerable
`std::error:Error` trait.

* contains a `ErrorKind` enum wrapping error types
* implements `From` conversions for the `?` operator to automatically perform conversions
* store the original error as a `Box<std::error::Error + Send + 'static>` object for chaining `cause`
* store a backtrace for tracing details

### Result <a name="result"/></a>
Having a simplified common `Result` type is  must for a Rust error handling solution. 

* support the ability to call a method on all `std::error::Error + Send + 'static` types
* collect and store a backtrace and the earliest opportunity

### wrap <a name="wrap"/></a>
Add additional context to an existing error is done with the `wrap` method implemented for both
the `std::error:Error` and the `std::result::Result`.

### bail! <a name="bail"/></a>
The `bail!` macro is a concept that came up again and again in the various error packages that I've
used and reviewed. The idea is simple. Make instantiating and returning a new error based on a string
simple and fast.

```rust
use witcher::prelude::*;

fn do_something() -> Result<()> {
    bail!("something bad")
}

fn main() {
    let err = do_something().unwrap_err();
    println!("{}", err);
}
```

### ensure! <a name="ensure"/></a>
The `ensure!` macro is another concept that is common in error handling. The intent is to allow
checking of parameters etc.. to quickly and easily create an error based on a message if a condition
is not met.

```rus
use witcher::prelude::*;

fn do_something(val: u32) -> Result<()> {
    ensure!(val > 2, "invalid argument")
}

fn main() {
    let err = do_something(1).unwrap_err();
    println!("{}", err);
}
```

### Terminate <a name="terminate"/></a>
This is a controversial subject but though some errors cannot be recovered from, in my opinion, all
errors can and should be handled. This means that all errors should propagate up the stack allowing
the full stack to participate in the handling of the error. There are a couple of reasons for this.
By percolating it up the stack additional context can be added to the chain of errors making it
easier to understand the path the code took and it's intent. Additionally it provides the ability to
have a single clean exit point in the code where detailed error logging can occur. For these reasons
functions like `unwrap` and `expect` should only be used in test code where error handling is
optional depending on what your testing.

`panic!` should be avoided:
```rust
use std::fs;

fn do_something() {
  panic!("Can't read Cargo.toml");
}

fn main() {
  do_something
}
```

`unwrap` should be avoided:
```rust
use std::fs;

fn do_something() {
  let content = fs::read_to_string("./Cargo.toml").unwrap();
  println!("{}", content)
}

fn main() {
  do_something
}
```

`expect` should be avoided:
```rust
use std::fs;

fn do_something() {
  let content = fs::read_to_string("./Cargo.toml").expect("Can't read Cargo.toml");
  println!("{}", content)
}

fn main() {
  do_something();
}
```

### Default value <a name="default-value"/></a>
In some cases it is enough to simply use a default value when an error occurs. I would suggest only
doing this when it is well ducumented and clear why it occured. In some cases a well documented
default value is perfectly acceptable.

Example:
```rust
use std::env;

fn main() {
  let level = env::var("LOG_LEVEL").unwrap_or("info".to_string());
  println!("{}", level);
}
```

### Coerce errors <a name="coerce-errors"/></a>
Implementing the `From` trait on custom errors allows the use of the `?` operator to convert other
errors into your error for error wrapping.

Example:
```rust
use std::{convert, error, fmt, io, result};

#[derive(Debug)]
pub struct CustomError {
    msg: String,
}
impl error::Error for CustomError {}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "CustomError: {}", self.msg)
    }
}

impl convert::From<io::Error> for CustomError {
    fn from(err: io::Error) -> Self {
        Self {
            msg: format!("{:?}", err),
        }
    }
}

fn do_something() -> result::Result<(), CustomError> {
    Err(io::Error::new(io::ErrorKind::Other, "oh no!"))?
}

fn main() {
    let err = do_something().unwrap_err();
    println!("{}", err.msg);
}
```

### Downcase errors <a name="downcast-errors"/></a>
When handling errors its often useful to take a boxed error and 

# Trait Objects <a name="trait-objects"/></a>
Typically most Rust discussion around `Dynamically Sized Types (DSTs)` and Traits generally use the
term `Traits` to discuss Rust's notion of interfaces and DST to discuss the quirks around Rust
implementation specifics of unsized types. I'm going to narrow in on `Traits` as it relates to DSTs
and use the term `Trait Object` to refer to this; which happens to also be a term used in the Rust
community to describe the `fat pointer` used to work with such instances. Often the terms get
conflated and used interchangably. At any rate I'll use the term to describe what I'm attempting to
do which is store an instance of concrete type in its generic `Trait` form inside a struct for later
hydration into its former concrete type. 

References:
* [DST Explanation](https://stackoverflow.com/questions/25740916/how-do-you-actually-use-dynamically-sized-types-in-rust)
* [DST, Take 5](https://smallcultfollowing.com/babysteps/blog/2014/01/05/dst-take-5/)
* [Safe DST Coercion](https://github.com/rust-lang/rfcs/blob/master/text/0982-dst-coercion.md)
* [Rustonomicon Book](https://doc.rust-lang.org/nomicon/index.html)
* [Type Conversions](https://doc.rust-lang.org/nomicon/conversions.html)
* [Casts](https://doc.rust-lang.org/nomicon/casts.html)
* [Transmutes](https://doc.rust-lang.org/nomicon/transmutes.html)
* [Dynamic Dispatch](https://alschwalm.com/blog/static/2017/03/07/exploring-dynamic-dispatch-in-rust/)
* [Trait Objects](https://doc.rust-lang.org/1.8.0/book/trait-objects.html)
* [Fat vs inner pointers](https://tratt.net/laurie/blog/entries/a_quick_look_at_trait_objects_in_rust.html)
* [Rust Fat Pointers](https://iandouglasscott.com/2018/05/28/exploring-rust-fat-pointers/)
* [Storing unboxed trait objects in Rust](https://guiand.xyz/blog-posts/unboxed-trait-objects.html)
* [Dynstack](https://github.com/archshift/dynstack/blob/master/src/lib.rs)
* [Rust Error Tutorial](https://benkay86.github.io/rust-error-tutorial.html)
* [std::io::Error](https://matklad.github.io/2020/10/15/study-of-std-io-error.html)

## Storing trait objects <a name="storing-trait-objects"/></a>
Storing trait object is best done in a Box or using lifetimes.

Example with lifetimes:
```rust
struct Foo<'a> {
  inner: &'a (dyn std::error::Error + Send + Sync + 'static)
}
```
Lifetimes wasn't ideal as it required a lifetime change to the `Result` alias I was building which
was a no go imo as it would affect all code that touched it.
```rust
type Result<T, E = Error> = std::result::Result<T, E>;
```

Exmample with Box:
```rust
struct Foo {
  inner: Box<dyn std::error::Error + Send + Sync + 'static>
}
```

## Fat pointers <a name="fat-pointers"/></a>
Trait objects are `fat pointers` meaning they are two words in size.  The first word is a pointer to
the actual `object data` and the second word is a pointer to the `vtable`. This fat pointer to our
trait type can be obtained by `casting` e.g. `&x as &Foo` or by `coercing` e.g. using `&x` as an
argument to a function that takes a `&Foo`.

This operation can be see as `erasing` the compiler's knoweledge about the specific type of the
pointer, and hence trait objects are sometimes referreed to as `type erasure`. A function that takes
a trait object without generic parameterization i.e. `<T>` will not create specialized functions for
each type saving code bloat, but will instead use a slower vtable to track the implementation
functions.

```rust
trait Foo {
  do_seomthing();
}

fn do_something(x: &Foo) {
    x.method();
}
fn main() {
    let x = 5u8;
    do_something(&x);         // coercion
    do_something(&x as &Foo); // casting
}
```


Coercing a trait object into a fat pointer will add the vtable and double the size:
```rust
use std::mem::size_of;

trait Foo { }

fn main() {
    assert_eq!(size_of::<&bool>(), size_of::<usize>());
    assert_eq!(size_of::<&dyn Foo>(), size_of::<usize>() * 2);
}
```


## Trait Pointer <a name="trait-pointer"/></a>
Instantiating a DST cannot be done directly instead it is done by coercing an existing instance
of a statically sized type into a DST. Essentially the compiler will erase static type information
and convert the static dispatch into a vtable and add it to a resulting fat pointer.

The smart pointer `std::rc::Rc` can be used as an example as it already has an implementation to do
the coercion from a sized to unsized type. In this example we create a statically typed version of
`Bar` and coerce it to the DST `Foo`.

```rust
use std::rc::Rc;

trait Foo {
    fn foo(&self) {
        println!("foo")
    }
}
struct Bar;
impl Foo for Bar {}

fn main() {
    let data: Rc<dyn Foo> = Rc::new(Bar);
    data.foo();
}
```

## Dynamic Dispatch <a name="dynamic-dispatch"/></a>
In `polymorphism` the mechanism to determine which version is run is called `dispatch`. While Rust
favors `static dispatch` it also supports dynamic dispatch through `trait objects`.

**Static Dispatch**:  
Rust uses `monomorphization` or `specialization` to perform static dispatch using trait bounds in the
following case.  This means that Rust will create a new version of `do_something` for each type used
and compile that in e.g. `do_something(x: String)` or `do_something(x: i32)` depending on what types
were used at compile time. The upside of this is its inline and fast, but bloats the binary with
extra copies of the same code for different types.
```rust
fn do_something<T: Foo>(x: T) {
    x.method();
}
```

**Dynamic Dispatch**:  
Dynamic dispatch is used for trait objects like `&Foo` or `Box<Foo>` where your storing a value of
any type that implements the given trait, where the precise type can only be known at runtime. A
trait object can be obtained from a pointer to a concrete type that implements the trait by `casting`
it e.g. `&x as &Foo` or `coercing` it e.g. using `&x` as an argument to a function that takes `&Foo`.

This operation can be see as `erasing` the compiler's knoweledge about the specific type of the
pointer, and hence trait objects are sometimes referreed to as `type erasure`. A function that takes
a trait object without generic parameterization i.e. `<T>` will not create specialized functions for
each type saving code bloat, but requiring slower virtual function calls by inhibiting inlining.
```rust
fn do_something(x: &Foo) {
    x.method();
}
fn main() {
    let x = 5u8;
    do_something(&x);         // coercion
    do_something(&x as &Foo); // casting
}
```

### vtable <a name="vtable"/></a>
The methods of the trait can be called on the trait object via a special record function pointers
traditionally called the `vtable` which is created and managed by the compiler. Trait objects are
both simple and complicated: their core representation and layout is quite straight forward, but
there are some error messages and surprising behavior to discover.

References:
* [\*const T vs \*mut T](https://internals.rust-lang.org/t/what-is-the-real-difference-between-const-t-and-mut-t-raw-pointers/6127)
* [nightly code around this](https://doc.rust-lang.org/1.8.0/std/raw/struct.TraitObject.html)
* [representation of a trait object](https://doc.rust-lang.org/1.8.0/book/trait-objects.html#representation)
* [Dynamic dispatch](https://alschwalm.com/blog/static/2017/03/07/exploring-dynamic-dispatch-in-rust/)

A trait like `&Foo` consists of a `data` pointer and a `vtable` pointer. The data pointer addresses
the data of some unknown type `T` that the trait object is storing, and the vtable pointer points to
the `virtual method table` corresponding to the implementation of `Foo` for `T`.
```rust
// Using *const to indicate that no change is being made to the data
struct TraitObject {
    data: *const (),
    vtable: *const (),
}
```

A vtable is essentially a struct of function pointers, pointing to the concrete piece of machine code
for each method in the implementation. A methdo call like `trait_object.method()` will retrieve the
correct pointer out of the vtble and then do a dynamic all of it. For example:
```rust
struct FooVtable {
    destructor: fn(*mut ()),
    size: usize,
    align: usize,
    method: fn(*const ()) -> String,
}

fn call_method_on_u8(x: *const ()) -> String {
    // the compiler guarantees that this function is only called with `x` pointing to a u8
    let byte: &u8 = unsafe { &*(x as *const u8) };
    byte.method()
}

static Foo_for_u8_vtable: FooVtable = FooVtable {
    destructor: /* compiler magic */,
    size: 1,
    align: 1,

    // cast to a function pointer
    method: call_method_on_u8 as fn(*const ()) -> String,
};
```

The `destructor` field in each vtable points to a function that will clean up any resources of the
vtable's type. For `u8` its trivial, but for `String` it will free the memory. This is necessary for
owning trait objects like `Box<Foo>` which need to clean-up both the `Box` allocation as well as the
internal type when they go out of scope. The `size` and `align` fields store the size of the erased
type and its alignment requirements; these are unused at the moment as the information is embedded in
the destructor, but will be used in the future as trait objects are made more flexible.

## Transmutting <a name="transmutting"/></a>
At the end of the day everything is just a pile of bits somewhere and the type systems are just there
to help us use those bits the right way. There are two common problems with typing bits: needing to
reinterpret those exact bits as a different type and needing to change the bits to have equivalent
meaning for a different type. Transmutation is used to manually reinterpret the underlying data
layout to perform our own typing. To be clear this is one of the most horribly unsafe things you can
do in Rust and the ways to cause undefined behavior are mind boggling.

`mem::transmute<T, U>` takes a value of type `T` and reinterpretes it to have a type `U`. The only
restriction is that the `T` and `U` are verified to have the same size.

Pitfalls:
* Creating an instance of any type with an invalid state is going to cause undeterminted chaos
* Not specifying the return type may cause chaos
* Transmuting a `&` to a `&mut` is no bueno
* Transmuting to a reference without an explicit lifetime produces an ubounded lifetime
* Compound types must have the same layout or the wrong fields will get the wrong data
  * Solution: use `repr(C)` which has precise data layout

Benefits:
* Essentially free performance wise

## Data layouts <a name="data-layouts"/></a>
The layout of a type is its `size`, `alignment` and the `relative offsets of its fields`. For enums,
how the discriminant is laid out and interpreted is also part of the type layout.

References:
* [Type layout](https://doc.rust-lang.org/reference/type-layout.html)
* [Data representations](https://doc.rust-lang.org/reference/type-layout.html#representations)
* [Alternative representations](https://doc.rust-lang.org/nomicon/other-reprs.html)
* [repr(C) structs](https://doc.rust-lang.org/reference/type-layout.html#reprc-structs)
* [Struct memory layout](https://doc.rust-lang.org/std/alloc/struct.Layout.html)

### Size and Alignment <a name="size-and-alignment"/></a>
All values have a size and alignment. The `alignment` of a value specifies what addresses are valid
to store the value at. A value of alignment `n` must only be stored at an address that is a multiple
of `n`. Alignment is measured in bytes and must be at least 1 and always a power of 2. The alignment
of a value can be checked with the `align_of_val` function.

The `size` of a value is the offset in bytes between successive elements in an array with that item
type including alignment padding. The size of a value is always a multiple of its alignment. The size
of a value can be checked with the `size_of_val` function.

`usize` and `isize` have a size big enough to contain every address on the target platform i.e. on a
32 bit target this value is `4 bytes` and on a 64 bit target this value is `8 bytes`.

Pointers to sized types have the same size and alignment as `usize`. Although you shouldn't rely on
this all pointers to DSTs are currently twice the size of `usize` and have the same alignment.

### repr(C) <a name="repr-c"/></a>
Rust allows you to specify alternative data layout strategies such as `repr(C)`.  This data
representation follows the C language specification for order, size and alignment of fields. Since
the ***default rust data representation has NO guarantees of data layout*** it is useful to use the
***repr(C) which does have guarantees of layout*** when passing types through the FFI boundary to C
and to form a sound base for more elaborate data layout manipulation such as ***reinterpreting values
as a different type***. 

Restrictions for using `repr(C)`
* ZSTs are still zero-sized
* DST pointers i.e. wide or fat pointers are not a concept in C and never FFI safe
* Can not be applied to `zero-variant enums` as there is no `C` representation

The representation of a type can be changed by applying the `repr` attribute. Additionally the
aligment may be altered as well.
```rust
// C representation, alignment raised to 8
#[repr(C, align(8))]
struct AlignedStruct {
    first: i16,
    second: i8,
    third: i32
}
```

As a consequence of the data layout representation being an attribute on the item, the representation
does not depend on generic parameters. This means that any two types with the same name regardless to
the generic's type have the same representation. For example both `Foo<Bar>` and `Foo<Baz>` for the
following code would both be `repr(C)`.

```rust
#[repr(C)]
struct Foo<T> {
    first: i16,
    second: i8,
    third: i32
}
```

For structs like those above the algorithm for determining size and offset is as follows:
1. Start with a current offset of 0 bytes
2. For each field in declaration order in the struct, first determine the size and alignment of the
   field. If the current offset is not a multiple of the field's alignment, then add padding to the
   current offset until it is a multiple of the field's alignment. the offset for the field is what
   the current offset is now. Then increase the current offset by teh size of the field.
3. Finally, the size of the struct is the current offset rounded up to the nearest multiple of the
   struct's alignment.
   

# Error Packages <a name="error-packages"/></a>
Rust has had a rough start with proper error handling. `Failure` was the first attempt to fix the
error handling issues in Rust. Since that time the `std::error::Error` trait has improved and now the
recommendation is to use [anyhow](https://github.com/dtolnay/anyhow) and [thiserror](https://crates.io/crates/thiserror)
both created by the same author. I still find the options lacking.

After reviewing dozens of implementations around errors it would seem that they generally fally into
two or three categories:
1. focus easily generating new error types via macros
2. focus on the handling of errors in a consistent way
3. some combination of the two

References:
* [Review of AnyHow](https://nick.groenen.me/posts/rust-error-handling)
* [Survey of error handling in Rust](https://blog.yoshuawuyts.com/error-handling-survey)

## Error Handling <a name="error-handling"/></a>
I'm grouping packages here that tend to focus on error handling or a more holistic approach rather
than on error generation.

### failure <a name="failure"/></a>
[failure](https://github.com/rust-lang-nursery/failure) now deprecated by the authors which now are
promoting the new in vogue packages `anyhow` and `thiserror`.

Failure was designed to make it easier to manage errors in Rust intended to address some of the
shortcommings perceived in `std::error::Error`.

Failure uses a new common struct `Error` type that they convert everything to similar to anyhow or
error-chain creatable from a `Box<dyn StdError + Sync + Send + 'static`>

### error-chain <a name="error-chain"/></a>
[error-chain](https://github.com/rust-lang-nursery/error-chain) billed as the predecessor and
inspiration `failure` seems to have fallen out of vogue sometime ago.

error-chain's principles as follows are interesting and resonated with some of my own requirements
for an error handling package.
* No error should ever be discarded
* Introducing new errors should be trivial
* Handling errors should be possible with pattern matching
* Conversions between error types should be done in an automatic and consistent way
* Errors should implement Send
* Errors should be able to carry backtraces

***new struct type Error***:
* contains a `ErrorKind` enum wrapping error types
* implements all the normal `From` conversions that let the `?` operatior work
* contains an opaque `Box<std::error::Error + Send + 'static>` object for chaining `cause`
* stores a Backtrace

***defines a new ResultExt trait***:
* defines a `chain_err` method for all `std::error::Error + Send + 'static` types
* uses `chain_err` to convert errors into `error_chain::Error` types and stores the original error in
a box.
* collects and stores a single Backtrace at the earliest opportunity and propogates it down the stack
through `From`.

***tradeoffs***
* Because the Error type contains a `std::error::Error + Send + 'static` it can't implement the
`PartialEq` for easy comparisons.

References:
* https://brson.github.io/2016/11/30/starting-with-error-chain

### anyhow <a name="anyhow"/></a>
[anyhow](https://github.com/dtolnay/anyhow) does a good job providing convenience and chaining of
errors. You can combine anyhow and tracing together to make a decent solution. I like that it is
built on top of the `std::error::Error` trait.

Running the code in this section will give you a nicely formatted output:
```bash
$ cargo run -q
Nov 28 15:54:10.384 ERROR cli: Failed: anyhow context for first thing

Caused by:
    0: anyhow context for second thing
    1: failed to do third thing
```

Although `anyhow` provides a chaining of contexts which is great and the simplicity of wrapping
errors to a common type which is two of the three areas for my minimal requirements I have yet to
understand how it surfaces the backtrace details.

Fails to meet requirements:
* simplified error tracing you'd find in something like the Golang errors package is missing
* the underlying error types are not printed out
* no good way to integrate the detailed error output with tracing json messaging formats

```rust
use anyhow::Context;
use std::{error::Error, fmt};
use tracing::error;

#[derive(Debug)]
struct FooError {
    message: &'static str,
}
impl FooError {
    fn new(message: &'static str) -> Self {
        Self { message }
    }
}
impl Error for FooError {}
impl fmt::Display for FooError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad(self.message)
    }
}

fn do_first_thing() -> anyhow::Result<()> {
    return do_second_thing().context("anyhow context for first thing");
}
fn do_second_thing() -> anyhow::Result<()> {
    return do_third_thing().context("anyhow context for second thing");
}
fn do_third_thing() -> anyhow::Result<()> {
    return Err(FooError::new("failed to do third thing"))?;
}

fn main() {
    tracing_subscriber::fmt().init();
    match do_first_thing() {
        Ok(_) => (),
        Err(e) => error!("Failed: {:?}", e),
    };
}
```

### tracing-error <a name="tracing-error"/></a>
[tracing-error](https://github.com/tokio-rs/tracing/tree/master/tracing-error) provides a way to
instrument errors with additional tracing information and is part of the venerable `tracing`
ecosystem from the `tokio` project. This is one of the few projects that provides tracing information
in their error infrastructure and is really close to what I was looking for. The implementation
actually uses `thiserror`

Backtracing example:
1. Clone the repo
2. Run the example
```bash
$ cargo run -q --example instrumented-error
Error 0: failed to do the additional thing
Backtrace:
   0: cli::do_something
           with foo="hello world"
             at src/main.rs:32
Backtrace:
   0: cli::do_another_thing
           with answer=42 will_succeed=true
             at src/main.rs:38
   1: cli::do_something
           with foo="hello world"
             at src/main.rs:32
```

Fails to meet requirements:
* too much configuration overhead
  * each function that participates in the error tracing needs to have tracing annotations
  * instrumenting the error uses an extremely verbose function name `.in_current_span()?`
* tracing information calls out the attribute location in the code not the actual code lines
  * the project is still unreleased and in beta so hopefully that improves


## Error Generation <a name="error-generation"/></a>
I'm grouping packages here that tend to focus on error generation.

### quick-errro <a name="quick-error"/></a>
[quick-error](https://github.com/tailhook/quick-error) is another macro based helper for quickly
generation new error types.

Fails to meet requirements:
* too much overhead introduced during compilation to be worth while for the boiler plate savings
* has no mechanism for getting tracing information

### error-type <a name="error-type "/></a>
[error-type](https://github.com/DanielKeep/rust-error-type) provides the `error_type!` macro to
assist in generating new error types. This seems quite similar to `thiserror`

Fails to meet requirements:
* too much overhead introduced during compilation to be worth while for the boiler plate savings
* has no mechanism for getting tracing information

### thiserror <a name="thiserror"/></a>
[thiserror](https://github.com/dtolnay/thiserror) allows you to quickly create wrapper errors with an
enumeration of error variants which according to the documentation are interchangable with
`std::error::Error` implementations written by hand. The crate makes is super simple to write error
boiler plate code. However there is a non-trivial amount of compilation overhead that comes with
this method that makes it a no go [see the reddit discussion](https://www.reddit.com/r/rust/comments/gj8inf/rust_structuring_and_handling_errors_in_2020/)

Fails to meet requirements:
* too much overhead introduced during compilation to be worth while
* has no mechanism for getting tracing information
