use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyList, PyTuple};
use pyo3::conversion::ToPyObject;
use mzdata::io::mgf::MGFReader;
use mzdata::io::MZFileReader; // For open_path
use mzdata::prelude::SpectrumLike; // Provides id(), peaks(), description(), etc.
use mzdata::params::ParamDescribed; // Provides get_param_by_name() if needed
use std::path::Path;
use numpy::IntoPyArray;

/// Load an MGF file and return a list of matchms Spectrum objects with clean metadata.
/// 
/// Each Spectrum is constructed as:
///   Spectrum(mz, intensities, metadata, metadata_harmonization=True)
/// where:
/// - `mz` and `intensities` are NumPy arrays built from Rust vectors (as f64),
/// - `metadata` is a dict containing:
///     - "id": the spectrum’s index (from the description),
///     - "precursor_mz": the precursor m/z value extracted from the precursor field.
/// 
/// In this version we access the precursor mass from the precursor field as follows:
///   If `desc.precursor` is Some, then we use the first ion’s `mz` value.
#[pyfunction]
fn load_from_mgf(py: Python, file_path: &str) -> PyResult<PyObject> {
    // Convert file_path into a PathBuf.
    let path_buf = Path::new(file_path).to_path_buf();
    
    // Open the MGF file.
    let reader = MGFReader::open_path(path_buf).map_err(|e| {
        pyo3::exceptions::PyIOError::new_err(format!("Failed to open MGF file: {:?}", e))
    })?;
    
    // Import the matchms module and retrieve the Spectrum class.
    let matchms_module = py.import("matchms")?;
    let spectrum_class = matchms_module.getattr("Spectrum")?;
    
    let mut spectra_objs = Vec::new();
    
    for spectrum in reader {
        // Collect m/z and intensity values.
        let mut mzs: Vec<f64> = Vec::new();
        let mut intensities: Vec<f64> = Vec::new();
        for peak in spectrum.peaks().iter() {
            mzs.push(peak.mz);
            intensities.push(peak.intensity as f64);
        }
        
        // Convert vectors into NumPy arrays.
        let np_mzs = mzs.into_pyarray(py);
        let np_intensities = intensities.into_pyarray(py);
        
        // Build a clean metadata dictionary.
        let metadata = PyDict::new(py);
        // Use the spectrum description's index as the id.
        let desc = spectrum.description();
        metadata.set_item("id", desc.index)?;
        
        // Extract the precursor m/z from the precursor field.
        if let Some(precursor) = &desc.precursor {
            if let Some(first_ion) = precursor.ions.first() {
                // Here we assume SelectedIon has a public field `mz` of type f64.
                metadata.set_item("precursor_mz", first_ion.mz)?;
            }
        }
        
        // Build the arguments tuple: (mz, intensities, metadata)
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

/// Count the number of spectra in an MGF file.
#[pyfunction]
fn count_spectra(file_path: &str) -> PyResult<usize> {
    let path_buf = Path::new(file_path).to_path_buf();
    let reader = MGFReader::open_path(path_buf).map_err(|e| {
        pyo3::exceptions::PyIOError::new_err(format!("Failed to open MGF file: {:?}", e))
    })?;
    Ok(reader.count())
}

/// A Python module implemented in Rust.
#[pymodule]
fn mgf_rs_io(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(count_spectra, m)?)?;
    m.add_function(wrap_pyfunction!(load_from_mgf, m)?)?;
    Ok(())
}
