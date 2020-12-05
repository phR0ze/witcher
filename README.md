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
  * [Concepts implemented](#concepts-implemented)
* [Error Handling](#error-handling)
  * [Terminate](#terminate)
  * [Default value](#default-value)
  * [Coerce errors](#coerce-errors)
  * [Downcast errors](#downcast-errors)
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
when a cryptic low level error is returned.

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

## Concepts implemented <a name="concepts-implemented"/></a>

### Error <a name="error"/></a>
Most error handling implementations have the concept of a new error type `Error` that will be used to
wrap other errors and contain more information that is available with the the venerable
`std::error:Error` trait.

* contains a `ErrorKind` enum wrapping error types
* implements `From` conversions for the `?` operator to automatically perform conversions
* store the original error as a `Box<std::error::Error + Send + 'static>` object for chaining `cause`
* store a backtrace for tracing details

### Result <a name="result"/></a>


* defines a `chain_err` method for all `std::error::Error + Send + 'static` types
* uses `chain_err` to convert errors into `error_chain::Error` types and stores the original error in
a box.
* collects and stores a single Backtrace at the earliest opportunity and propogates it down the stack
through `From`.

***tradeoffs***
* Because the Error type contains a `std::error::Error + Send + 'static` it can't implement the
`PartialEq` for easy comparisons.

### bail! <a name="bail"/></a>
The `bail!` macro is a concept that came up again and again in the various error packages that I've
used and reviewed. The idea is simple. Make instantiating and returning a new error based on a string
simple and fast.

```rust
tbd
```

## Error Handling <a name="error-handling"/></a>

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
