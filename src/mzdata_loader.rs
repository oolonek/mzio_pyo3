// src/mzdata_loader.rs

use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyList, PyTuple};
use pyo3::conversion::ToPyObject;
use std::path::Path;
use numpy::IntoPyArray;

// Bring the trait that provides `open_path` into scope.
use mzdata::io::MZFileReader;
use mzdata::io::mgf::MGFReader;
use mzdata::prelude::SpectrumLike; // Provides peaks(), description(), etc.

#[pyfunction]
pub fn load_mg_with_mzdata(py: Python, file_path: &str) -> PyResult<PyObject> {
    // Convert file_path into a PathBuf.
    let path_buf = Path::new(file_path).to_path_buf();

    // Open the MGF file using mzdata.
    let reader = MGFReader::open_path(path_buf).map_err(|e| {
        pyo3::exceptions::PyIOError::new_err(format!("Failed to open MGF file: {:?}", e))
    })?;
    
    // Import the matchms module and retrieve the Spectrum class.
    let matchms_module = py.import("matchms")?;
    let spectrum_class = matchms_module.getattr("Spectrum")?;
    
    let mut spectra_objs = Vec::new();
    
    // Iterate over each spectrum in the file.
    for spectrum in reader {
        let mut mzs: Vec<f64> = Vec::new();
        let mut intensities: Vec<f64> = Vec::new();
        for peak in spectrum.peaks().iter() {
            mzs.push(peak.mz);
            intensities.push(peak.intensity as f64);
        }
        
        // Convert Rust vectors into NumPy arrays.
        let np_mzs = mzs.into_pyarray(py);
        let np_intensities = intensities.into_pyarray(py);
        
        // Build a metadata dictionary.
        let metadata = PyDict::new(py);
        let desc = spectrum.description();
        metadata.set_item("id", desc.index)?;
        
        // Extract precursor m/z if available.
        if let Some(precursor) = &desc.precursor {
            if let Some(first_ion) = precursor.ions.first() {
                metadata.set_item("precursor_mz", first_ion.mz)?;
            }
        }
        
        // Build the arguments tuple: (mzs, intensities, metadata)
        let args = PyTuple::new(py, &[
            np_mzs.to_object(py),
            np_intensities.to_object(py),
            metadata.to_object(py),
        ]);
        let kwargs = [("metadata_harmonization", true)].into_py_dict(py);
        
        // Create the Spectrum object.
        let spectrum_obj = spectrum_class.call(args, Some(kwargs))?;
        spectra_objs.push(spectrum_obj);
    }
    
    // Return a Python list of Spectrum objects.
    Ok(PyList::new(py, spectra_objs).to_object(py))
}

#[pyfunction]
pub fn count_spectra(file_path: &str) -> PyResult<usize> {
    let path_buf = Path::new(file_path).to_path_buf();
    let reader = MGFReader::open_path(path_buf).map_err(|e| {
        pyo3::exceptions::PyIOError::new_err(format!("Failed to open MGF file: {:?}", e))
    })?;
    Ok(reader.count())
}
