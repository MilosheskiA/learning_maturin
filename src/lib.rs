use pyo3::prelude::*;

mod data;

/// Replicates the behavior of the python calc_time function
#[pyfunction]
fn calc_time_rust() -> PyResult<()> {
    // Create a vector with 100,000,000 integers
    let long_list_to_measure: Vec<i32> = (0..100_000_000).collect();
    println!("Calculated list of length: {}", long_list_to_measure.len());
    println!("done for now");
    Ok(())
}

/// This function receives a CarDataFromPython struct and prints its contents.
/// Args:
///     car_info (data::CarDataFromPython): The car data received from Python.
/// Returns:
///     PyResult<()>: An empty result indicating successful execution.

#[pyfunction]
fn car_data(car_info: data::CarDataFromPython) -> PyResult<String> {
    println!("{:?}", car_info);
    let year = car_info.year;

    Ok(format!("Car year is: {}", year))
}

/// A Python module implemented in Rust.
#[pymodule]
fn celery_rust_calc(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calc_time_rust, m)?)?;
    m.add_function(wrap_pyfunction!(car_data, m)?)?;
    Ok(())
}
