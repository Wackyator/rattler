mod create_env;
mod error;
mod match_spec;
mod nameless_match_spec;
mod repo_data;
mod version;

use create_env::{
    fetch_repo_data, find_installed_packages, PyAuthenticatedClient, PyFetchRepoDataOptions,
};
use error::{
    InvalidMatchSpecException, InvalidPackageNameException, InvalidVersionException, IoException,
    PyRattlerError,
};
use match_spec::PyMatchSpec;
use nameless_match_spec::PyNamelessMatchSpec;
use repo_data::package_record::PyPackageRecord;
use version::PyVersion;

use pyo3::prelude::*;

#[pymodule]
fn rattler(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<PyVersion>().unwrap();

    m.add_class::<PyMatchSpec>().unwrap();
    m.add_class::<PyNamelessMatchSpec>().unwrap();

    m.add_class::<PyPackageRecord>().unwrap();

    m.add_class::<PyFetchRepoDataOptions>().unwrap();
    m.add_class::<PyAuthenticatedClient>().unwrap();

    m.add_function(wrap_pyfunction!(find_installed_packages, m).unwrap())
        .unwrap();
    m.add_function(wrap_pyfunction!(fetch_repo_data, m).unwrap())
        .unwrap();

    // Exceptions
    m.add(
        "InvalidVersionError",
        py.get_type::<InvalidVersionException>(),
    )
    .unwrap();
    m.add(
        "InvalidMatchSpecError",
        py.get_type::<InvalidMatchSpecException>(),
    )
    .unwrap();
    m.add(
        "InvalidPackageNameError",
        py.get_type::<InvalidPackageNameException>(),
    )
    .unwrap();
    m.add("IoError", py.get_type::<IoException>()).unwrap();

    Ok(())
}
