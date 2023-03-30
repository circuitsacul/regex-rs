use regex::Captures as ReCaptures;
use pyo3::prelude::*;
use self_cell::self_cell;

use crate::match_struct::Match;

self_cell! {
    #[pyclass]
    pub struct Captures {
        owner: String,

        #[covariant]
        dependent: ReCaptures,
    }

    impl {Debug}
}

#[pymethods]
impl Captures {
    pub fn get(&self, i: usize) -> Option<Match> {
        self.borrow_dependent().get(i).map(|m| m.into())
    }

    pub fn name(&self, name: &str) -> Option<Match> {
        self.borrow_dependent().name(name).map(|m| m.into())
    }

    pub fn expand(&self, replacement: &str, mut dst: String) -> String {
        self.borrow_dependent().expand(replacement, &mut dst);
        dst
    }

    pub fn __iter__(&self) {
        todo!()
    }

    pub fn __len__(&self) -> usize {
        self.borrow_dependent().len()
    }

    pub fn __repr__(&self) -> String {
        let dep = self.borrow_dependent();
        format!("{dep:#?}")
    }
}
