use crate::term;
use crate::misc::OmitExt;
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
    "witcher::backtrace::simple",
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
];

// Simplify backtrace to pull out the important information.
pub fn simple() -> String {
    let mut w = String::new();
    let bt = backtrace::Backtrace::new();

    // Process backtrace frames
    bt.frames().iter().flat_map(|x| x.symbols()).for_each(|sym| {
        if !is_dependency(sym) {

            // Write out the symbol name (i.e. function, method, module etc...)
            match sym.name() {
                Some(name) => writeln!(w, "{:#}", name).omit(),
                None => writeln!(w, "<unknown>").omit(),
            };

            // Write out the shortened simplified path
            writeln!(&mut w, "{}", simple_path(sym.filename(), sym.lineno())).omit();
        }
    });

    if term::isatty() {
        //write!(&mut w, "{:?}", bt).omit();
    }

    return w
}

// Write out a shortened simplified path if possible
fn simple_path(filename: Option<&Path>, lineno: Option<u32>) -> String {
    let mut w = String::new();
    if let (Some(file), Some(line)) = (filename, lineno) {

        // Strip off the current working directory to simplify the path
        let cwd = std::env::current_dir();
        if let Ok(cwd) = &cwd {
            if let Ok(suffix) = file.strip_prefix(cwd) {
                write!(w, "  at {}:{}", suffix.display(), line).omit();
                return w
            }
        }
        write!(w, "  at {}:{}", file.display(), line).omit();
    }
    return w
}

// Check if the given symbol name or file name is a known rust dependency
fn is_dependency(sym: &backtrace::BacktraceSymbol) -> bool {
    if let Some(name) = sym.name() {
        let str = format!("{:#}", name);
        if DEPENDENCY_SYM_PREFIXES.iter().any(|x| str.starts_with(x)) {
            return true;
        }
    }
    if let Some(filename) = sym.filename() {
        let filename = filename.to_string_lossy();
        if DEPENDENCY_FILE_PREFIXES.iter().any(|x| filename.starts_with(x))
            || DEPENDENCY_FILE_CONTAINS.iter().any(|x| filename.contains(x))
        {
            return true;
        }
    }
    false
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_simple_path() {
        let cwd = std::env::current_dir().unwrap();
        assert_eq!("  at foo:10", simple_path(Some(Path::new(&cwd).join("foo").as_ref()), Some(10)));
        assert_eq!("  at foobar:10", simple_path(Some(Path::new(&cwd).join("foobar").as_ref()), Some(10)));
        assert_eq!("  at /rustc/123/src/libstd/foobar:10", simple_path(Some(Path::new("/rustc/123/src/libstd").join("foobar").as_ref()), Some(10)));
    }

}