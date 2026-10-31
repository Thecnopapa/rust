use pyo3::prelude::*;
mod base;

use pyo3::types::{PyDict, PyString};
use pyo3::PyTypeInfo;

#[pyclass()]
struct MMCIF;


#[pymethods]
impl MMCIF {
    #[new]
    #[pyo3(signature = (filepath))]
    fn __new__(
        filepath: &Bound<'_, PyString>
    ) -> PyResult<Self> {
        Ok(Self)
    }

    #[pyo3(signature = (filepath))]
    fn __init__(
        slf: &Bound<'_, Self>,
        filepath: &Bound<'_, PyString>
    ) -> PyResult<()> {
        Ok(())
    }
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn print(filepath:String) -> PyResult<String> {
    base::parser::read(filepath.clone());
    return Ok(filepath.clone())
}

#[pyfunction]
fn parse(filepath:String, include:Vec<String>, exclude:Vec<String>) -> PyResult<String> {
    base::parser::parse(filepath.clone(), include.clone(), exclude.clone());
    return Ok(filepath)
}

/// A Python module implemented in Rust.
#[pymodule]
fn mmcif_parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(print, m)?)?;
    m.add_function(wrap_pyfunction!(parse, m)?)?;
    m.add_class::<MMCIF>()?;
    Ok(())
}