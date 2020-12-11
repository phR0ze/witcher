use libc;
use colored::*;
use std::fmt::Display;

/// detect if we are using a tty
pub fn isatty() -> bool {
    unsafe { libc::isatty(libc::STDOUT_FILENO) != 0 }
}

pub struct Colorized {
    colorized: bool,
}
impl Colorized {
    pub fn new() -> Self {
        Self { colorized: isatty() }
    }

    pub fn red<M>(&self, msg: M) -> ColoredString
    where
        M: Display
    {
        match self.colorized {
            true => format!("{}", msg).bright_red(),
            _ => ColoredString::default(),
        }
    }

    pub fn cyan<M>(&self, msg: M) -> ColoredString
    where
        M: Display
    {
        match self.colorized {
            true => format!("{}", msg).bright_cyan(),
            _ => ColoredString::default(),
        }
    }

}