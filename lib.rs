use pyo3::prelude::*;

/// Replicates the behavior of the python calc_time function
#[pyfunction]
fn calc_time_rust() -> PyResult<()> {
    // Create a vector with 100,000,000 integers
    let long_list_to_measure: Vec<i32> = (0..100_000_000).collect();
    println!("Calculated list of length: {}", long_list_to_measure.len());
    println!("done for now");
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn celery_rust_calc(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calc_time_rust, m)?)?;
    Ok(())
}
