use pyo3::prelude::*;

pub struct RegexError(regex::Error);
pub type RegexResult<T> = Result<T, RegexError>;

impl From<RegexError> for PyErr {
    fn from(value: RegexError) -> Self {
        PyErr::new::<pyo3::types::PyString, _>(value.0.to_string())
    }
}

impl From<regex::Error> for RegexError {
    fn from(value: regex::Error) -> Self {
        Self(value)
    }
}
