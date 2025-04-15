// Shamelessly copied from the PyO3 website.

use pyo3::prelude::*;

#[pyfunction]
fn double(x: usize) -> usize {
    x * 2
}

/// This module is implemented in Rust.
#[pymodule]
fn example(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(double, m)?)
}
