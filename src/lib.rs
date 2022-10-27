use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn compute_cid(data: Vec<u8>) -> PyResult<String> {
    let mut adder = ipfs_unixfs::file::adder::FileAdder::default();
    let size_hint = adder.size_hint();

    let mut written: usize = 0;
    while written < data.len() {
        let end = written + (data.len() - written).min(size_hint);
        let slice = &data[written..end];

        let (_blocks, pushed) = adder.push(slice);
        written += pushed;
    }

    let last_blocks = adder.finish();
    let (cid, _block) = last_blocks.last().unwrap();

    Ok(cid.to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_ipfs_cid(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(compute_cid, m)?)?;
    Ok(())
}
