use pyo3::{exceptions::PyValueError, prelude::*};

pub struct RegexError(regex::Error);

impl From<RegexError> for PyErr {
    fn from(value: RegexError) -> Self {
        PyValueError::new_err(value.0.to_string())
    }
}

impl From<regex::Error> for RegexError {
    fn from(value: regex::Error) -> Self {
        Self(value)
    }
}
