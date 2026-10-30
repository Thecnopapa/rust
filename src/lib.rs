use pyo3::prelude::*;
mod base;

/// Formats the sum of two numbers as string.
#[pyfunction]
crate::base::parser::parse;

/// A Python module implemented in Rust.
#[pymodule]
fn mmcif_parser(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(crate::base::parser::parse, m)?)?;
    m.add_function(wrap_pyfunction!(crate::base::parser::read, m)?)?;

    Ok(())
}