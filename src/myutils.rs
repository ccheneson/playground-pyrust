use pyo3::prelude::*;


#[pyfunction]
pub fn multi_and_sum(x: Vec<u64>, y: Vec<u64>) -> PyResult<u64> {
    let mut result: u64 = 0;
    for i in &x {
        for j in &y {
            result = result + (i * j);
        }
    }
    Ok(result)
}


pub fn register(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(multi_and_sum, m)?)
    
}

#[cfg(test)]
mod tests {
    use crate::myutils::multi_and_sum;

    #[test]
    fn it_works() {
        let result = multi_and_sum(vec![1, 2, 3], vec![3, 4, 5]);
        match result {
            Ok(u64) => assert_eq!(u64, 72, "Wrong result from multi_and_sum"),
            Err(err) => assert!(false, "An error returned from multi_and_sum : {}", err),
        }
    }
}
