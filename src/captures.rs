use std::sync::Arc;

use ouroboros::self_referencing;
use pyo3::prelude::*;
use regex::Captures as ReCaptures;

use crate::match_struct::Match;

#[pyclass]
#[self_referencing(pub_extras)]
pub struct Captures {
    pub text: Arc<String>,

    #[borrows(text)]
    #[covariant]
    pub captures: ReCaptures<'this>,
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

    pub fn __getitem__(&self, i: usize) -> Option<Match> {
        self.get(i)
    }

    pub fn __len__(&self) -> usize {
        self.borrow_captures().len()
    }

    pub fn __repr__(&self) -> String {
        let dep = self.borrow_captures();
        format!("{dep:#?}")
    }
}
