use std::error::Error;
use std::fmt;

/// Errors produced by fallible `Greenback` operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GreenbackError {
    /// `cents` passed to a constructor was outside `0..=99`.
    InvalidCents(i64),
    /// An arithmetic operation would have overflowed `i64`.
    Overflow,
    /// A division or remainder operation had a zero divisor.
    DivideByZero,
    /// A string could not be parsed into a `Greenback`.
    ParseError(String),
}

impl fmt::Display for GreenbackError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GreenbackError::InvalidCents(cents) => {
                write!(f, "cents must be between 0 and 99, got {cents}")
            }
            GreenbackError::Overflow => write!(f, "arithmetic operation overflowed"),
            GreenbackError::DivideByZero => write!(f, "attempted to divide by zero"),
            GreenbackError::ParseError(s) => write!(f, "could not parse {s:?} as a dollar amount"),
        }
    }
}

impl Error for GreenbackError {}
