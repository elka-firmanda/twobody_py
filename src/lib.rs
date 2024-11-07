use pyo3::prelude::*;
mod compute;
pub use compute::simulate_moon_earth;

#[pyfunction]
fn simulate_moon_earth_py() -> Vec<Vec<f64>> {
    simulate_moon_earth()
}

#[pymodule]
fn twobody_py(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(simulate_moon_earth_py, m)?)?;
    Ok(())
}
