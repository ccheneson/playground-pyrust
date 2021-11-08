use pyo3::prelude::*;


#[pyfunction]
pub fn multi_and_sum(x: Vec<u64>, y: Vec<u64>) -> PyResult<u64> {
    let mut result: u64 = 0;
    for i in x.iter() {
        for j in y.iter() {
            result = result + (i * j);
        }
    }
    Ok(result)
}

#[pymodule]
fn myutils(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(multi_and_sum, m)?)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::multi_and_sum;

    #[test]
    fn it_works() {
        let result = multi_and_sum(vec![1, 2, 3], vec![3, 4, 5]);
        match result {
            Ok(u64) => assert_eq!(u64, 72, "Wrong result from multi_and_sum"),
            Err(err) => assert!(false, "An error returned from multi_and_sum : {}", err),
        }
    }
}
