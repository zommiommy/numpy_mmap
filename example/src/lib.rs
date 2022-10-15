use pyo3::prelude::*;
use numpy_mmap::{Dtype, create_numpy_array};

/// C
#[pyfunction]
fn zeros(path: String) -> PyResult<String> {
    let embedding = create_numpy_array(
        gil.python(),
        self.get_path().as_ref().map(|x| x.as_str()),
        $dtype_enum,
        &[rows_number, columns_number],
        true,
    );

    let s = embedding.cast_as::<PyArray2<$dtype>>(gil.python())?;

    let embedding_slice = unsafe { s.as_slice_mut()? };

}

#[pymodule]
fn example(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(zeros, m)?)?;
    Ok(())
}