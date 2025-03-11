// src/lib.rs

mod mzdata_loader;
mod mascotrs_loader;

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

/// A Python module implemented in Rust.
#[pymodule]
fn mgf_rs_io(_py: Python, m: &PyModule) -> PyResult<()> {
    // mzdata-based functions.
    m.add_function(wrap_pyfunction!(mzdata_loader::load_mg_with_mzdata, m)?)?;
    m.add_function(wrap_pyfunction!(mzdata_loader::count_spectra, m)?)?;
    
    // mascot_rs-based function.
    m.add_function(wrap_pyfunction!(mascotrs_loader::load_mgf_with_mascotrs, m)?)?;
    Ok(())
}
