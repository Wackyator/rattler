use pyo3::exceptions::PyException;
use pyo3::{create_exception, PyErr};
use rattler_conda_types::{InvalidPackageNameError, ParseMatchSpecError, ParseVersionError};
use rattler_repodata_gateway::fetch::FetchRepoDataError;
use thiserror::Error;

#[derive(Error, Debug)]
#[allow(clippy::enum_variant_names)]
pub enum PyRattlerError {
    #[error(transparent)]
    InvalidVersion(#[from] ParseVersionError),
    #[error(transparent)]
    InvalidMatchSpec(#[from] ParseMatchSpecError),
    #[error(transparent)]
    InvalidPackageName(#[from] InvalidPackageNameError),
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error(transparent)]
    FetchRepoDataError(#[from] FetchRepoDataError),
    #[error("Unknown Error")]
    Unknown,
}

impl From<PyRattlerError> for PyErr {
    fn from(value: PyRattlerError) -> Self {
        match value {
            PyRattlerError::InvalidVersion(err) => {
                InvalidVersionException::new_err(err.to_string())
            }
            PyRattlerError::InvalidMatchSpec(err) => {
                InvalidMatchSpecException::new_err(err.to_string())
            }
            PyRattlerError::InvalidPackageName(err) => {
                InvalidPackageNameException::new_err(err.to_string())
            }
            PyRattlerError::IoError(err) => IoException::new_err(err.to_string()),
            PyRattlerError::FetchRepoDataError(err) => {
                FetchRepoDataException::new_err(err.to_string())
            }
            PyRattlerError::Unknown => {
                UnknownException::new_err(PyRattlerError::Unknown.to_string())
            }
        }
    }
}

create_exception!(exceptions, InvalidVersionException, PyException);
create_exception!(exceptions, InvalidMatchSpecException, PyException);
create_exception!(exceptions, InvalidPackageNameException, PyException);
create_exception!(exceptions, IoException, PyException);
create_exception!(exceptions, FetchRepoDataException, PyException);
create_exception!(exceptions, UnknownException, PyException);
