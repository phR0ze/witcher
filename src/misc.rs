
pub trait OmitExt {
    /// Simply consumes the result ignoring it
    fn omit(&self);
}

impl OmitExt for std::fmt::Result {
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
    use std::fmt::Write;
    
    #[test]
    fn test_the() {
        let mut w = String::new();
        write!(&mut w, "foobar").omit();
    }
}