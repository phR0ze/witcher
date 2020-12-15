use colored::*;
use libc;
use std::env;
use std::fmt::Display;

const NO_COLOR: &str = "0";
pub const WITCHER_COLOR: &str = "WITCHER_COLOR";

pub fn isatty() -> bool {
    unsafe { libc::isatty(libc::STDOUT_FILENO) != 0 }
}

pub struct Colorized {
    colorized: bool,
}

impl Colorized {
    pub fn new() -> Self {
        let mut colorized = isatty();
        if &*env::var(WITCHER_COLOR).unwrap_or("1".to_string()) == NO_COLOR {
            colorized = false;
        }
        Self { colorized }
    }

    pub fn red<M>(&self, msg: M) -> ColoredString
    where
        M: Display,
    {
        match self.colorized {
            true => format!("{}", msg).bright_red(),
            _ => format!("{}", msg).bright_red().clear(),
        }
    }

    pub fn cyan<M>(&self, msg: M) -> ColoredString
    where
        M: Display,
    {
        match self.colorized {
            true => format!("{}", msg).bright_cyan(),
            _ => format!("{}", msg).bright_cyan().clear(),
        }
    }
}
