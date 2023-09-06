use std::{path::PathBuf, str::FromStr, sync::Arc};

use futures::{stream::FuturesUnordered, StreamExt};
use pyo3::{pyclass, pyfunction, pymethods, PyAny, PyResult, Python};
use rattler_conda_types::PrefixRecord;
use rattler_networking::AuthenticatedClient;
use rattler_repodata_gateway::fetch::{self, CachedRepoData, FetchRepoDataOptions};
use reqwest::{Method, RequestBuilder};
use tokio::task::JoinHandle;
use url::Url;

use crate::error::PyRattlerError;

#[pyclass]
#[repr(transparent)]
#[derive(Clone)]
pub struct PyPrefixRecord {
    pub(crate) inner: PrefixRecord,
}

impl From<PrefixRecord> for PyPrefixRecord {
    fn from(value: PrefixRecord) -> Self {
        Self { inner: value }
    }
}

impl From<PyPrefixRecord> for PrefixRecord {
    fn from(value: PyPrefixRecord) -> Self {
        value.inner
    }
}

#[pymethods]
impl PyPrefixRecord {
    pub fn write_to_path(&self, path: PathBuf, pretty: bool) -> PyResult<()> {
        Ok(self.inner.clone().write_to_path(path, pretty)?)
    }

    #[staticmethod]
    pub fn from_path(path: PathBuf) -> PyResult<Self> {
        let inner = PrefixRecord::from_path(path)?;
        Ok(PyPrefixRecord { inner })
    }
}

#[pyclass]
#[repr(transparent)]
#[derive(Clone)]
pub struct PyAuthenticatedClient {
    pub(crate) inner: AuthenticatedClient,
}

#[pymethods]
impl PyAuthenticatedClient {
    #[new]
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get(&self, url: String) -> PyRequestBuilder {
        self.inner.get(url).into()
    }

    pub fn post(&self, url: String) -> PyRequestBuilder {
        self.inner.post(url).into()
    }

    pub fn head(&self, url: String) -> PyRequestBuilder {
        self.inner.head(url).into()
    }

    pub fn request(&self, method: String, url: String) -> PyResult<PyRequestBuilder> {
        if let Ok(m) = Method::from_str(method.as_ref()) {
            Ok(self.inner.request(m, url).into())
        } else {
            Err(PyRattlerError::Unknown.into())
        }
    }
}

impl Default for PyAuthenticatedClient {
    fn default() -> Self {
        AuthenticatedClient::default().into()
    }
}

impl From<AuthenticatedClient> for PyAuthenticatedClient {
    fn from(value: AuthenticatedClient) -> Self {
        Self { inner: value }
    }
}

impl From<PyAuthenticatedClient> for AuthenticatedClient {
    fn from(value: PyAuthenticatedClient) -> Self {
        value.inner
    }
}

#[pyclass]
#[repr(transparent)]
#[derive(Clone)]
pub struct PyRequestBuilder {
    pub(crate) inner: Arc<RequestBuilder>,
}

impl From<RequestBuilder> for PyRequestBuilder {
    fn from(value: RequestBuilder) -> Self {
        Self {
            inner: Arc::new(value),
        }
    }
}

impl From<PyRequestBuilder> for RequestBuilder {
    fn from(value: PyRequestBuilder) -> Self {
        Arc::<RequestBuilder>::into_inner(value.inner)
            .expect("Inner value has multiple strong references?")
    }
}

#[pyclass]
#[repr(transparent)]
#[derive(Clone)]
pub struct PyFetchRepoDataOptions {
    pub(crate) inner: FetchRepoDataOptions,
}

#[pymethods]
impl PyFetchRepoDataOptions {
    #[new]
    pub fn new(func: Option<&PyAny>) -> PyResult<Self> {
        if let Some(_func) = func {
            // if !func.is_callable() {
            //     return Err(PyRattlerError::Unknown.into());
            // }
            // Ok(Self {
            //     inner: FetchRepoDataOptions {
            //         download_progress: Some(func),
            //         ..Default::default()
            //     },
            // })
            Ok(Self::default())
        } else {
            Ok(Self::default())
        }
    }
}

impl Default for PyFetchRepoDataOptions {
    fn default() -> Self {
        FetchRepoDataOptions::default().into()
    }
}

impl From<FetchRepoDataOptions> for PyFetchRepoDataOptions {
    fn from(value: FetchRepoDataOptions) -> Self {
        Self { inner: value }
    }
}

impl From<PyFetchRepoDataOptions> for FetchRepoDataOptions {
    fn from(value: PyFetchRepoDataOptions) -> Self {
        value.inner
    }
}

#[pyclass]
#[repr(transparent)]
#[derive(Clone, Debug)]
pub struct PyCachedRepoData {
    pub(crate) inner: Arc<CachedRepoData>,
}

#[pymethods]
impl PyCachedRepoData {
    pub fn as_str(&self) -> String {
        format!("{:?}", &self.inner)
    }
}

impl From<CachedRepoData> for PyCachedRepoData {
    fn from(value: CachedRepoData) -> Self {
        Self {
            inner: Arc::new(value),
        }
    }
}

impl From<PyCachedRepoData> for CachedRepoData {
    fn from(value: PyCachedRepoData) -> Self {
        Arc::<CachedRepoData>::into_inner(value.inner)
            .expect("Inner value has multiple strong references?")
    }
}

// subdir_url: https://conda.anaconda.org/conda-forge/linux-64/
// cache_path: /home/toaster/.cache/rattler/cache/repodata
#[pyfunction]
pub fn fetch_repo_data(
    py: Python,
    subdir_url: String,
    client: PyAuthenticatedClient,
    cache_path: PathBuf,
    options: PyFetchRepoDataOptions,
) -> PyResult<&PyAny> {
    let v = pyo3_asyncio::tokio::future_into_py::<_, PyCachedRepoData>(py, async move {
        if let Ok(url) = Url::parse(&subdir_url) {
            let res = fetch::fetch_repo_data(url, client.into(), &cache_path, options.into()).await;
            match res {
                Ok(v) => Ok(PyCachedRepoData::from(v)),
                Err(e) => Err(PyRattlerError::FetchRepoDataError(e).into()),
            }
        } else {
            Err(PyRattlerError::Unknown.into())
        }
    });
    v
}

#[pyfunction]
pub fn find_installed_packages(
    py: Python,
    target_prefix: PathBuf,
    concurrency_limit: usize,
) -> PyResult<&PyAny> {
    pyo3_asyncio::tokio::future_into_py(py, async move {
        let mut meta_futures =
            FuturesUnordered::<JoinHandle<Result<PrefixRecord, std::io::Error>>>::new();
        let mut result: Vec<PyPrefixRecord> = Vec::new();
        for entry in std::fs::read_dir(target_prefix.join("conda-meta"))
            .into_iter()
            .flatten()
        {
            let entry = entry?;
            let path = entry.path();
            if path.ends_with(".json") {
                continue;
            }

            // If there are too many pending entries, wait for one to be finished
            if meta_futures.len() >= concurrency_limit {
                match meta_futures
                    .next()
                    .await
                    .expect("we know there are pending futures")
                {
                    Ok(record) => result.push(record?.into()),
                    Err(e) => {
                        if let Ok(panic) = e.try_into_panic() {
                            std::panic::resume_unwind(panic);
                        }
                        // The future was cancelled, we can simply return what we have.
                        return Ok(result);
                    }
                }
            }

            // Spawn loading on another thread
            let future = tokio::task::spawn_blocking(move || PrefixRecord::from_path(path));
            meta_futures.push(future);
        }

        while let Some(record) = meta_futures.next().await {
            match record {
                Ok(record) => result.push(record?.into()),
                Err(e) => {
                    if let Ok(panic) = e.try_into_panic() {
                        std::panic::resume_unwind(panic);
                    }
                    // The future was cancelled, we can simply return what we have.
                    return Ok(result);
                }
            }
        }

        Ok(result)
    })
}
