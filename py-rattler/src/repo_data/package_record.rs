use rattler_conda_types::{
    package::{IndexJson, PackageFile},
    PackageName, PackageRecord,
};
use std::path::PathBuf;

use pyo3::{pyclass, pymethods, PyResult};

use crate::{error::PyRattlerError, version::PyVersion};

#[pyclass]
#[repr(transparent)]
#[derive(Clone)]
pub struct PyIndexJson {
    pub(crate) inner: IndexJson,
}

#[pymethods]
impl PyIndexJson {
    #[new]
    pub fn new(json_string: String) -> PyResult<Self> {
        Ok(Self {
            inner: IndexJson::from_str(json_string.as_str())?,
        })
    }

    #[staticmethod]
    pub fn from_package_directory(path: PathBuf) -> PyResult<Self> {
        Ok(Self {
            inner: IndexJson::from_package_directory(path)?,
        })
    }

    #[staticmethod]
    pub fn from_path(path: PathBuf) -> PyResult<Self> {
        Ok(Self {
            inner: IndexJson::from_path(path)?,
        })
    }
}

#[pyclass]
#[repr(transparent)]
#[derive(Clone)]
pub struct PyPackageName {
    pub(crate) inner: PackageName,
}

#[pyclass]
#[repr(transparent)]
#[derive(Clone)]
pub struct PyPackageRecord {
    pub(crate) inner: PackageRecord,
}

#[pymethods]
impl PyPackageName {
    #[new]
    pub fn new(normalised_name: String) -> Self {
        Self {
            inner: PackageName::new_unchecked(normalised_name),
        }
    }
}

impl From<PackageRecord> for PyPackageRecord {
    fn from(value: PackageRecord) -> Self {
        Self { inner: value }
    }
}

impl From<PyPackageRecord> for PackageRecord {
    fn from(val: PyPackageRecord) -> Self {
        val.inner
    }
}

#[pymethods]
impl PyPackageRecord {
    #[new]
    pub fn new(name: PyPackageName, version: PyVersion, build: String) -> PyPackageRecord {
        Self {
            inner: PackageRecord::new(name.inner, version.inner, build),
        }
    }

    #[staticmethod]
    pub fn from_index_json(
        index: &PyIndexJson,
        size: Option<u64>,
        sha256: Option<String>,
        md5: Option<String>,
    ) -> PyResult<Self> {
        let p = PackageRecord::from_index_json(index.inner, size, None, None);

        match p {
            Ok(v) => Ok(Self { inner: v }),
            Err(e) => Err(PyRattlerError::ConvertSubdirError(e).into()),
        }
    }

    #[staticmethod]
    pub fn sort_topologically(records: Vec<Self>) -> Vec<Self> {
        todo!()
    }

    /// Returns a string representation of PyPackageRecord
    pub fn as_str(&self) -> String {
        format!("{}", self.inner)
    }
}

// Extracts Source and PyVersion from `VersionWithSource` in python
// pub fn get_version_and_source(version: &PyAny) -> PyResult<(String, PyVersion)> {
//     let py = version.py();

//     let repr = version
//         .getattr(intern!(py, "_source"))?
//         .downcast::<PyString>()?
//         .to_string();

//     let inner_version = version.getattr(intern!(py, "_version"))?;

//     if !inner_version.is_instance_of::<PyVersion>() {
//         return Err(PyRattlerError::Unknown.into());
//     }

//     let inner_version = inner_version.extract::<PyVersion>()?;

//     Ok((repr, inner_version))
// }
