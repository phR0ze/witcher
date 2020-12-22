use crate::WITCHER_COLOR;
use colored::*;
use std::env;
use std::ffi::OsStr;
use std::fmt::Display;

/// Determine if the environment has an attached tty
pub fn isatty() -> bool {
    unsafe { libc::isatty(libc::STDOUT_FILENO) != 0 }
}

/// Check if the given environment variable is enabled or disabled.
/// If the env var is not set, `false` or `0` it will be reported as disabled all other
/// values will be reported as true.
pub fn var_enabled<K: AsRef<OsStr>>(key: K) -> bool {
    !matches!(env::var(key).unwrap_or_else(|_| "false".to_string()).to_lowercase().as_str(), "false" | "0")
}

/// Check if the given environment variable is enabled or disabled.
/// If the env var is not set, `false` or `0` it will be reported as disabled all other
/// values will be reported as true.
/// Supports setting the given default if not set.
pub fn var_enabled_d<K: AsRef<OsStr>>(key: K, default: &str) -> bool {
    !matches!(env::var(key).unwrap_or_else(|_| default.to_string()).to_lowercase().as_str(), "false" | "0")
}

pub struct Colorized {
    colorized: bool,
}

impl Colorized {
    pub fn new() -> Self {
        Self { colorized: if isatty() { var_enabled_d(WITCHER_COLOR, "true") } else { false } }
    }

    pub fn red<M>(&self, msg: M) -> ColoredString
    where
        M: Display,
    {
        if self.colorized {
            format!("{}", msg).bright_red()
        } else {
            format!("{}", msg).bright_red().clear()
        }
    }

    pub fn cyan<M>(&self, msg: M) -> ColoredString
    where
        M: Display,
    {
        if self.colorized {
            format!("{}", msg).bright_cyan()
        } else {
            format!("{}", msg).bright_cyan().clear()
        }
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    // Disable backtrace
    use std::sync::Once;
    static INIT: Once = Once::new();
    pub fn initialize() {
        INIT.call_once(|| {
            env::set_var("RUST_BACKTRACE", "0");
        });
    }

    #[test]
    fn test_var_enabled_d() {
        initialize();
        assert!(!var_enabled_d("BLAH", "false"));
        assert!(var_enabled_d("BLAH", "true"));
    }

    #[test]
    fn test_enabled() {
        initialize();
        assert!(!var_enabled("FOOBAR"));

        // Test true case
        env::set_var("FOOBAR", "true");
        assert!(var_enabled("FOOBAR"));
        env::set_var("FOOBAR", "True");
        assert!(var_enabled("FOOBAR"));
        env::set_var("FOOBAR", "blah");
        assert!(var_enabled("FOOBAR"));

        // Test false case
        env::set_var("FOOBAR", "0");
        assert!(!var_enabled("FOOBAR"));
        env::set_var("FOOBAR", "false");
        assert!(!var_enabled("FOOBAR"));
        env::set_var("FOOBAR", "False");
        assert!(!var_enabled("FOOBAR"));
    }
}
