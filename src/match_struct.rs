use std::sync::Arc;

use ouroboros::self_referencing;
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

#[pyclass]
#[self_referencing(pub_extras)]
pub struct Matches {
    text: String,
    re: Arc<regex::Regex>,

    #[borrows(text, re)]
    #[not_covariant]
    matches: regex::Matches<'this, 'this>,
}

#[pymethods]
impl Matches {
    pub fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    pub fn __next__(&mut self) -> Option<Match> {
        self.with_matches_mut(|iter| iter.next().map(|m| m.into()))
    }

    pub fn __repr__(&self) -> String {
        self.with_matches(|matches| format!("{matches:#?}"))
    }
}
