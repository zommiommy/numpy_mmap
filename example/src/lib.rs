use pyo3::prelude::*;
use numpy::PyArray2;
use numpy_mmap::{Dtype, create_numpy_array};

/// Create a numpy 2D matrix filled with ones
#[pyfunction]
fn ones(py: Python, path: String, rows_number: isize, columns_number: isize) -> PyResult<Py<PyAny>> {
    // create the array
    let embedding = create_numpy_array(
        py,
        Some(path.as_str()),
        Dtype::F32,
        &[rows_number, columns_number],
        true,
    );

    // cast it to a pyarray of f32, this is not automatically done by the previous
    // method so you can write generic functions
    let s = embedding.cast_as::<PyArray2<f32>>(py)?;

    // get a rust-compatible slice
    let embedding_slice = unsafe { s.as_slice_mut()? };

    // initialize it with ones
    embedding_slice.iter_mut().for_each(|x| {*x = 1.0;});
    
    Ok(embedding)
}

#[pymodule]
fn example(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(ones, m)?)?;
    Ok(())
}