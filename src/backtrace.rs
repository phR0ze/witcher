use std::fmt::Write;
use std::path::Path;

const DEPENDENCY_FILE_PREFIXES: &[&str] = &[
    "/rustc/",
    "src/libstd/",
    "src/libpanic_unwind/",
    "src/libtest/",
];

const DEPENDENCY_FILE_CONTAINS: &[&str] = &[
    "/.cargo/registry/src/",
];

const DEPENDENCY_SYM_PREFIXES: &[&str] = &[
    "std::",
    "core::",
    "witcher::error::",
    "<witcher::error::",
    "witcher::backtrace::",
    "backtrace::backtrace::",
    "_rust_begin_unwind",
    "color_traceback::",
    "__rust_",
    "___rust_",
    "__pthread",
    "_main",
    "main",
    "__scrt_common_main_seh",
    "BaseThreadInitThunk",
    "_start",
    "__libc_start_main",
    "start_thread",
    "__GI__",
];

const DEPENDENCY_SYM_CONTAINS: &[&str] = &[
    "as witcher::wrapper::Wrapper",
];


// Process the given backtrace return a simplified Frame collection
pub(crate) fn new() -> Vec<Frame> {
    let bt = backtrace::Backtrace::new();

    bt.frames().iter().flat_map(|x| x.symbols()).map(|sym| {
        Frame {
            symbol: match sym.name() {
                Some(name) => format!("{:#}", name),
                None => String::from("<unknown>"),
            },
            filename: simple_path(sym.filename()),
            lineno: sym.lineno(),
            column: sym.colno(),
        }
    }).collect()
}

// Provide a convenient way to work with frame information
#[derive(Debug)]
pub(crate) struct Frame {
    pub symbol: String,         // name of the symbol or '<unknown>'
    pub filename: String,       // filename the symbole occurred in
    pub lineno: Option<u32>,    // line number the symbol occurred on
    pub column: Option<u32>,    // column number the symbol occurred on
}
impl Frame {

    // Check if this is a known rust dependency
    pub fn is_dependency(&self) -> bool {
        if DEPENDENCY_SYM_PREFIXES.iter().any(|x| self.symbol.starts_with(x))
            || DEPENDENCY_SYM_CONTAINS.iter().any(|x| self.symbol.contains(x))
            || DEPENDENCY_FILE_PREFIXES.iter().any(|x| self.filename.starts_with(x))
            || DEPENDENCY_FILE_CONTAINS.iter().any(|x| self.filename.contains(x))
        {
            return true;
        }
        false
    }
}

// Write out a shortened simplified path if possible
fn simple_path(filename: Option<&Path>) -> String {
    let mut w = String::new();
    if let Some(file) = filename {

        // Strip off the current working directory to simplify the path
        let cwd = std::env::current_dir();
        if let Ok(cwd) = &cwd {
            if let Ok(suffix) = file.strip_prefix(cwd) {
                write!(w, "{}", suffix.display()).omit();
                return w
            }
        }
        write!(w, "{}", file.display()).omit();
        return w
    }
    write!(w, "<unknown>").omit();
    return w
}

// Helper to suppress unwanted result checks
// -------------------------------------------------------------------------------------------------
trait Omit {
    fn omit(&self);
}
impl Omit for std::fmt::Result {
     fn omit(&self) {
         let _ = match self {
             Ok(_) => (),
             Err(_) => (),
         };
     }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
     
    #[test]
    fn test_omit() {
        let mut w = String::new();
        write!(&mut w, "foobar").omit();
    } 

    #[test]
    fn test_simple_path() {
        let cwd = std::env::current_dir().unwrap();
        assert_eq!("foo", simple_path(Some(Path::new(&cwd).join("foo").as_ref())));
        assert_eq!("foobar", simple_path(Some(Path::new(&cwd).join("foobar").as_ref())));
        assert_eq!("/rustc/123/src/libstd/foobar", simple_path(Some(Path::new("/rustc/123/src/libstd").join("foobar").as_ref())));
        assert_eq!("<unknown>", simple_path(None));
    }
}