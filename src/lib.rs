mod regex;
mod error;
mod capture_locations;
mod match_struct;
mod captures;

use pyo3::prelude::*;

/// A Python module implemented in Rust.
#[pymodule]
fn regex_rs(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<regex::Regex>()?;
    m.add_class::<capture_locations::CaptureLocations>()?;
    m.add_class::<match_struct::Match>()?;
    m.add_class::<captures::Captures>()?;
    Ok(())
}