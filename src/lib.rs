use pyo3::prelude::*;
use cid::Cid;

#[pyclass]
#[derive(Debug, Clone)]
enum CidVersion {
    V0,
    V1,
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn compute_cid(data: Vec<u8>, cid_version: Option<CidVersion>) -> PyResult<String> {
    let mut adder = unixfs_v1::file::adder::FileAdder::default();
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

    println!("CID version: {:?}", cid_version);

    let cid = match cid_version.unwrap_or(CidVersion::V0) {
        CidVersion::V0 => Cid::new_v0(*cid.hash()).unwrap(),
        CidVersion::V1 => cid.into_v1().unwrap(),
    };

    Ok(cid.to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn py_ipfs_cid(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<CidVersion>()?;
    m.add_function(wrap_pyfunction!(compute_cid, m)?)?;
    Ok(())
}
