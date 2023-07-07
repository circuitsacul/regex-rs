use std::sync::Arc;

use ouroboros::self_referencing;
use pyo3::prelude::*;

#[pyclass]
#[self_referencing(pub_extras)]
pub struct Split {
    pub text: String,
    pub re: Arc<regex::Regex>,
    pub limit: Option<usize>,

    #[borrows(text, re)]
    #[not_covariant]
    pub split: regex::Split<'this, 'this>,
}

#[pymethods]
impl Split {
    pub fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    pub fn __next__(&mut self) -> Option<&str> {
        let ret = self.with_limit_mut(|limit| {
            if let Some(limit) = limit {
                if *limit == 0 {
                    return false;
                }

                *limit -= 1;
            }

            true
        });
        if !ret {
            return None;
        }

        self.with_split_mut(|split| split.next())
    }
}
