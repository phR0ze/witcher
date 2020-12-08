
// Following the std::raw::TraitObject pattern rather than using it since it is nightly-only.
// https://doc.rust-lang.org/std/raw/struct.TraitObject.html
// https://doc.rust-lang.org/src/core/raw.rs.html
//
// This struct has the same internal layout as Trait DSTs e.g. &dyn Wrapper and Box<dyn Wrapper>
// allowing you to deconstruct a Trait DST into its raw constituent parts that will no longer be
// managed by the Rust memory management system and needs to be handled manually.
//
// TraitObject is guaranteed to match layouts by using the alternate repr(C) data representation to
// get a reliable layout and thus can be used as targets for transmutes in unsafe code for
// manipulating the raw representations directly. The only way to create values of this type is with functions like 
// std::mem::transmute. Similarly, the only way to create a true trait object from a TraitObject 
// value is with transmute.
//
// *const and *mut are equivalent in this context. I'm using *const to indicate that no change
// is going to occur to the error's raw constituent parts.
#[repr(C)]
struct TraitObject {
    data: *const (),
    vtable: *const (),
}

// Wrap an error's raw constituent parts with this wrapper so that we can easily refer to them 
// individually and manage memory manually while still being able to convert back into the
// original typed error when needed. This will allow for managing any error type that implements
// the trait std::error::Error and for complex operations like matching on error types.
// Using alternate repr(C) data representation to get a reliable layout.
#[repr(C)]
struct Wrapper<T> {
    vtable: *const (),      // the original error's virtual method table
    error: T,               // the original error
}
impl Wrapper<()> {
    // Re-construct the original error from its raw constituent parts
    fn error(&self) -> &(dyn std::error::Error + Send + Sync + 'static) {
        unsafe {
            // https://doc.rust-lang.org/src/core/raw.rs.html#69
            std::mem::transmute(TraitObject {
                data: &self.error,      // gets coerced into *const type
                vtable: self.vtable,
            })
        }
    }
}

pub struct Error {
    // Wrapper<()> is a ZST or zero sized type in this context acting as a place holder for an
    // unmanaged concrete instance of Wrapper. This means memory will need to be manually managed.
    wrapper: Box<Wrapper<()>>,
}
impl std::error::Error for Error { }
impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.wrapper.error())
    }
}
impl Error {

    /// Wrap the given error for internal use.
    /// 
    /// We require bounding with Send, Sync and 'static to ensure that the low level type
    /// manipulation being done internally will be as safe as possible.
    pub fn new<T>(err: T) -> Error 
    where 
        T: std::error::Error + Send + Sync + 'static,
    {
        unsafe {
            // Deconstruct the error Trait DST into is various raw constituent parts
            // https://doc.rust-lang.org/src/core/raw.rs.html#60
            let obj: TraitObject =  std::mem::transmute(&err as &dyn std::error::Error);

            // Construct a wrapper around the error's raw components for internal access
            let wrapper = Wrapper {
                vtable: obj.vtable,
                error: err,
            };

            // Construct a public facing general error encapsulating all this detail
            Error {
                // Transmute the wrapper to ensure it is unmanaged and strip off its typing
                wrapper: std::mem::transmute(Box::new(wrapper)),
            }
        }
    }
}

fn main() {
    let err = std::io::Error::new(std::io::ErrorKind::Other, "oh no!");
    println!("{}", err);
    let err = Error::new(err);
    println!("{}", err);
    println!("{}", err);
}
