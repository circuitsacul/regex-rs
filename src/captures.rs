use std::sync::Arc;

use ouroboros::self_referencing;
use pyo3::{exceptions::PyIndexError, prelude::*};

use crate::match_struct::Match;

#[pyclass]
#[self_referencing(pub_extras)]
pub struct Captures {
    pub text: Arc<String>,

    #[borrows(text)]
    #[covariant]
    pub captures: regex::Captures<'this>,
}

#[pymethods]
impl Captures {
    pub fn get(&self, i: usize) -> Option<Match> {
        self.borrow_captures().get(i).map(|m| m.into())
    }

    pub fn name(&self, name: &str) -> Option<Match> {
        self.borrow_captures().name(name).map(|m| m.into())
    }

    pub fn expand(&self, replacement: &str, mut dst: String) -> String {
        self.borrow_captures().expand(replacement, &mut dst);
        dst
    }

    pub fn __getitem__(&self, i: usize) -> PyResult<Match> {
        self.get(i).ok_or(PyIndexError::new_err(i))
    }

    pub fn __len__(&self) -> usize {
        self.borrow_captures().len()
    }

    pub fn __repr__(&self) -> String {
        format!("{:#?}", self.borrow_captures())
    }
}

#[pyclass]
#[self_referencing(pub_extras)]
pub struct CapturesIter {
    pub text: Arc<String>,
    pub re: Arc<regex::Regex>,

    #[borrows(text, re)]
    #[not_covariant]
    pub capture_matches: regex::CaptureMatches<'this, 'this>,
}

#[pymethods]
impl CapturesIter {
    pub fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    pub fn __next__(&mut self) -> Option<Captures> {
        let text = self.borrow_text().clone();
        self.with_capture_matches_mut(|iter| {
            iter.next()
                .map(|caps| Captures::new(text, |text| caps.adopt(text)))
        })
    }

    pub fn __repr__(&self) -> String {
        self.with_capture_matches(|caps| format!("{caps:#?}"))
    }
}
