// src/mascotrs_loader.rs

use pyo3::prelude::*;
use pyo3::types::{IntoPyDict, PyDict, PyList, PyTuple};
use pyo3::conversion::ToPyObject;
use numpy::IntoPyArray;

// Bring in the MGFVec type and related traits from mascot_rs.
use mascot_rs::prelude::{MGFVec, MascotGenericFormat};

#[pyfunction]
pub fn load_mgf_with_mascotrs(py: Python, file_path: &str) -> PyResult<PyObject> {
    // Load the MGF file using mascot_rs's MGFVec.
    let mgf_vec: MGFVec<usize, f32> = MGFVec::from_path(file_path).map_err(|e| {
        pyo3::exceptions::PyIOError::new_err(format!(
            "Failed to open MGF file with mascot_rs: {:?}",
            e
        ))
    })?;
    
    // Import the matchms module and retrieve the Spectrum class.
    let matchms_module = py.import("matchms")?;
    let spectrum_class = matchms_module.getattr("Spectrum")?;
    
    let mut spectra_objs = Vec::new();
    
    // For each MascotGenericFormat (spectrum) in the vector...
    for mgf in mgf_vec.into_vec() {
        // Use the first fragmentation level from each MascotGenericFormat.
        let first_level = mgf.get_first_fragmentation_level().map_err(|e| {
            pyo3::exceptions::PyValueError::new_err(format!(
                "Failed to get first fragmentation level: {}",
                e
            ))
        })?;
        
        // Collect m/z and intensity values.
        let mzs: Vec<f64> = first_level
            .mass_divided_by_charge_ratios_iter()
            .copied()
            .map(|mz| mz as f64)
            .collect();
        let intensities: Vec<f64> = first_level
            .fragment_intensities_iter()
            .copied()
            .map(|i| i as f64)
            .collect();
            
        // Convert vectors into NumPy arrays.
        let np_mzs = mzs.into_pyarray(py);
        let np_intensities = intensities.into_pyarray(py);
        
        // Build a metadata dictionary.
        let metadata = PyDict::new(py);
        // Use the feature id as the "id".
        metadata.set_item("id", mgf.feature_id())?;
        // Use the parent ion mass as the "precursor_mz".
        metadata.set_item("precursor_mz", mgf.parent_ion_mass() as f64)?;
        
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
