use crate::{Error, Result, StdError};
use std::any::TypeId;

/// Define the `wrap` function for Result types
pub trait Wrapper<T, E> {
    /// Wrap the error providing the ability to add more context
    fn wrap(self, msg: &str) -> Result<T>;

    /// Check if there is an error and the err is the given error type
    fn err_is<U>(&self) -> bool
    where
        U: StdError + 'static;

    /// Retry the given function when we have an error `max` number of times.
    fn retry<F>(self, max: usize, f: F) -> Result<T, E>
    where
        F: Fn(usize) -> Result<T, E>;

    /// Retry the given function when we have the concreate error `U` `max` number of times.
    fn retry_on<F>(self, max: usize, id: TypeId, f: F) -> Result<T, E>
    where
        F: Fn(usize) -> Result<T, E>;
}

impl<T, E> Wrapper<T, E> for Result<T, E>
where
    E: StdError + Send + Sync + 'static,
{
    fn wrap(self, msg: &str) -> Result<T> {
        match self {
            Err(err) => Error::wrap(err, msg),
            Ok(val) => Ok(val),
        }
    }

    fn err_is<U>(&self) -> bool
    where
        U: StdError + 'static,
    {
        match self {
            Ok(_) => false,
            Err(e) => (e as &(dyn StdError + 'static)).is::<U>(),
        }
    }

    fn retry<F>(self, max: usize, f: F) -> Result<T, E>
    where
        F: Fn(usize) -> Result<T, E>,
    {
        let mut retries = 0;
        let mut result = self;
        while retries < max && result.is_err() {
            retries += 1;
            result = f(retries);
        }
        result
    }

    fn retry_on<F>(self, max: usize, id: TypeId, f: F) -> Result<T, E>
    where
        F: Fn(usize) -> Result<T, E>,
    {
        let mut retries = 0;
        let mut result = self;
        while retries < max
            && match result {
                Ok(_) => false,
                Err(_) => TypeId::of::<E>() == id,
            }
        {
            retries += 1;
            result = f(retries);
        }
        result
    }
}

// Unit tests
// -------------------------------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::Once;
    static INIT: Once = Once::new();
    pub fn initialize() {
        INIT.call_once(|| {
            std::env::set_var(gory::TERM_COLOR, "0");
            std::env::set_var("RUST_BACKTRACE", "0");
        });
    }

    fn retry() -> Result<()> {
        do_external_thing().retry(3, |_| do_external_thing()).wrap("Failed while attacking beast")
    }

    fn retry_on_concreate_error_type_using_err_is() -> Result<()> {
        let mut retries = 0;
        let mut result = do_external_thing();
        while retries < 3 && result.err_is::<std::io::Error>() {
            retries += 1;
            result = do_external_thing();
        }
        result.wrap(&format!("Failed while attacking beast: {}", retries))
    }

    fn retry_on_concreate_error_type() -> Result<()> {
        do_external_thing().retry_on(3, TypeId::of::<std::io::Error>(), |_| do_external_thing()).wrap("Failed while attacking beast")
    }

    fn do_external_thing() -> std::io::Result<()> {
        Err(std::io::Error::new(std::io::ErrorKind::Other, "Oh no, we missed!"))
    }

    #[test]
    fn test_retry_on() {
        initialize();
        assert_eq!("Failed while attacking beast", retry().unwrap_err().to_string());
        assert_eq!("Failed while attacking beast", retry_on_concreate_error_type().unwrap_err().to_string());
        assert_eq!("Failed while attacking beast: 3", retry_on_concreate_error_type_using_err_is().unwrap_err().to_string());
    }
}
