# witcher
Track and put down errors

I intend to either investigate and document the error handling solution I'll use for Rust or
implement my own depending on my findings.

### Table of Contents
* [Background](#background)
  * [Requirements](#requirements)
  * [Error Packages](#error-packages)
    * [anyhow](#anyhow)
    * [tracing-error](#tracing-error)
    * [thiserror](#thiserror)
 
# Background <a name="background"/></a>
Rust has had a rough start with proper error handling. `Failure` was the first attempt to fix the
error handling issues in Rust. Since that time the `std::error::Error` trait has improved and now the
recommendation is to use [anyhow](https://github.com/dtolnay/anyhow) and [thiserror](https://crates.io/crates/thiserror)
both created by the same author. I still find the options lacking.

## Requirements <a name="requirements"/></a>
From my perspective errors should capture enough meta data around a problem to give a developer
a good chance of finding the error's origin without having to try and reproduce the problem. They
should be convenient enough to work with that developers will actually use them. Finally they should
add minimal overhead that will impact performance or developers won't use them.

This means errors:
* shouldn't require complicated configuration
  * error handling should be as concise and terse as possible
* should to able to interact with and wrap other errors for aggregation
  * provide a chaining of errors to provide context and detail
* should provide enough detail that the source of the error is evident
  * provide optional tracing in certain modes to allow for source line discovery

### Error Packages <a name="error-packages"/></a>

References:
* https://nick.groenen.me/posts/rust-error-handling
* https://blog.yoshuawuyts.com/error-handling-survey

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

### thiserror <a name="thiserror"/></a>
[thiserror](https://github.com/dtolnay/thiserror) allows you to quickly create wrapper errors with an
enumeration of error variants which according to the documentation are interchangable with
`std::error::Error` implementations written by hand. The crate makes is super simple to write error
boiler plate code. However there is a non-trivial amount of compilation overhead that comes with
this method that makes it a no go [see the reddit discussion](https://www.reddit.com/r/rust/comments/gj8inf/rust_structuring_and_handling_errors_in_2020/)

Fails to meet requirements:
* too much overhead introduced during compilation to be worth while
* has no mechanism for getting tracing information
