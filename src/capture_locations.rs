use pyo3::prelude::*;

#[pyclass]
pub struct CaptureLocations(regex::CaptureLocations);

#[pymethods]
impl CaptureLocations {
    fn get(&self, i: usize) -> Option<(usize, usize)> {
        self.0.get(i)
    }

    fn __len__(&self) -> usize {
        self.0.len()
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }
}

impl From<regex::CaptureLocations> for CaptureLocations {
    fn from(value: regex::CaptureLocations) -> Self {
        Self(value)
    }
}
