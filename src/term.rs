use libc;
use colored::*;
use std::env;
use std::fmt::Display;

const NO_COLOR: &str = "0";

/// detect if we are using a tty
pub fn isatty() -> bool {
    unsafe { libc::isatty(libc::STDOUT_FILENO) != 0 }
}

/// Provides colored with a tty aware wrapper and an option to disable
/// colors.
pub struct Colorized {
    colorized: bool,
}
impl Colorized {
    pub fn new() -> Self {
        let mut colorized = isatty();
        if &*env::var("COLOR").unwrap_or("1".to_string()) == NO_COLOR {
            colorized = false;
        }
        Self { colorized }
    }

    pub fn red<M>(&self, msg: M) -> ColoredString
    where
        M: Display
    {
        match self.colorized {
            true => format!("{}", msg).bright_red(),
            _ => format!("{}", msg).bright_red().clear(),
        }
    }

    pub fn cyan<M>(&self, msg: M) -> ColoredString
    where
        M: Display
    {
        match self.colorized {
            true => format!("{}", msg).bright_cyan(),
            _ => format!("{}", msg).bright_cyan().clear(),
        }
    }

}