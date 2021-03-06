use std::convert::From;
use std::error;
use std::fmt;
use std::num::ParseIntError;

use ansi_term::{ANSIString, Color};
use config::{Config, Value};

/// Type that will be returned when a module is formatted
#[derive(Debug, Default)]
pub struct FormatResult {
    pub output: Option<ANSIString<'static>>,
    pub next_bg: Option<Color>,
}

/// Struct representation of an Error encountered in the program
#[derive(Debug, PartialEq)]
pub struct Error {
    kind: ErrorKind,
    message: String,
}

/// Representation of specific errors that can be encountered
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ErrorKind {
    /// Input in config was not one of the allowed types
    InvalidTypeInConfig,
    /// Input in config didn't correspond to anything meaningful
    NoSuchMatchInConfig,
    /// Input in config was strictly malformed and couldn't be parsed
    ConfigParseFailure,
}

impl Error {
    pub fn new(kind: ErrorKind, msg: &str) -> Error {
        Error {
            kind: kind,
            message: msg.to_string(),
        }
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.message
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}: {}", self.kind, self.message)
    }
}

// So that we can use try!() and ? to return early if we encounter a
// ParseIntError
impl From<ParseIntError> for Error {
    fn from(e: ParseIntError) -> Error {
        use std::error::Error;
        self::Error::new(ErrorKind::InvalidTypeInConfig, e.description())
    }
}

/// Gets an array from a config file using a key.
///
/// Returns `None` if the key wasn't present or couldn't be coerced
/// into a `Vec<Value>`.
///
/// `Config::get_array()` in version `0.4.1` has a bug where it
/// consumes `self` instead of taking `self` by reference. This
/// function is a workaround for the time being. The bug has been
/// fixed but will not be available until the release of version `0.5`
/// of `config-rs`.
pub fn ref_get_array(key: &str, config: &Config) -> Option<Vec<Value>> {
    config.get(key).and_then(Value::into_array)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // The real test is that this compiles successfully without giving
    // a warning about the config file being moved.
    fn test_ref_get_array() {
        let mut c: Config = Config::new();
        c.set("numbers", vec![1, 2, 3]).unwrap();
        c.set("boolean", true).unwrap();

        // Uncomment and the compiler should complain
        // assert_eq!(c.get_array("numbers"),
        //            Some(vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)]));
        // assert_eq!(c.get_array("boolean"), None); // Use of moved value: c

        assert_eq!(ref_get_array("numbers", &c),
                   Some(vec![Value::Integer(1), Value::Integer(2), Value::Integer(3)]));
        assert_eq!(ref_get_array("boolean", &c), None);
    }
}
