use pyo3::prelude::*;

#[pyfunction]
fn fibonacci(n: u32) -> PyResult<u64> {
    if n > 93 {
        return Err(pyo3::exceptions::PyValueError::new_err(
            "Fibonacci(n) is too large for u64 when n > 93"
        ));
    }
    match n {
        0 => Ok(0),
        1 => Ok(1),
        _ => {
            let mut a = 0u64;
            let mut b = 1u64;
            for _ in 2..=n {
                let temp = a.checked_add(b)
                    .ok_or_else(|| pyo3::exceptions::PyOverflowError::new_err("u64 overflow"))?;
                a = b;
                b = temp;
            }
            Ok(b)
        }
    }
}

#[pymodule]
fn pyrust_demo(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(fibonacci, m)?)?;
    Ok(())
}