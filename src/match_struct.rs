use pyo3::prelude::*;

#[pyclass]
#[derive(Debug)]
pub struct Match {
    pub matched_text: String,
    pub start: usize,
    pub end: usize,
}

impl From<regex::Match<'_>> for Match {
    fn from(value: regex::Match) -> Self {
        Self {
            matched_text: value.as_str().to_owned(),
            start: value.start(),
            end: value.end(),
        }
    }
}

#[pymethods]
impl Match {
    fn __str__(&self) -> &str {
        &self.matched_text
    }

    fn __repr__(&self) -> String {
        format!("{self:#?}")
    }
}
