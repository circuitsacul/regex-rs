use pyo3::prelude::*;

#[pyclass]
#[derive(Debug)]
pub struct Match {
    #[pyo3(get)]
    pub matched_text: String,
    #[pyo3(get)]
    pub start: usize,
    #[pyo3(get)]
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
