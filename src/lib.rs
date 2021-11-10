pub mod myutils;

use pyo3::prelude::*;



#[pymodule]
fn rustils(py: Python, m: &PyModule) -> PyResult<()> {
    myutils::register(py, m)?;
    Ok(()) 
}
