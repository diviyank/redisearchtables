use pyo3::{
    types::{PyBool, PyBytes, PyDict, PyFloat, PyInt, PyList, PyModule, PySet, PyString, PyTuple},
    IntoPy, Py, PyAny, PyObject, PyResult, Python, ToPyObject,
};


/// Add submodule.
///
/// This function is required,
/// because by default for native libs python
/// adds module as an attribute and
/// doesn't add it's submodules in list
/// of all available modules.
///
/// To surpass this issue, we
/// maually update `sys.modules` attribute,
/// adding all submodules.
///
/// # Errors
///
/// May result in an error, if
/// cannot construct modules, or add it,
/// or modify `sys.modules` attr.
pub fn add_submodule(
    py: Python<'_>,
    parent_mod: &PyModule,
    name: &'static str,
    module_constuctor: impl FnOnce(Python<'_>, &PyModule) -> PyResult<()>,
) -> PyResult<()> {
    let sub_module = PyModule::new(py, name)?;
    module_constuctor(py, sub_module)?;
    parent_mod.add_submodule(sub_module)?;
    py.import("sys")?
        .getattr("modules")?
        .set_item(format!("{}.{name}", parent_mod.name()?), sub_module)?;
    Ok(())
}

/// Small function to integrate custom result type
/// and `pyo3_asyncio`.
///
/// It's almost the same as `future_into_py`,
/// but it expects future to return `ScyllaPyResult` type, rather
/// than `PyResult` from `pyo3`. It's useful for using `?` operators all over the place.
///
/// # Errors
///
/// If result of a future was unsuccessful, it propagates the error.
pub fn redis_future<F, T>(py: Python<'_>, fut: F) -> RediSearchResult<&PyAny>
where
    F: Future<Output = RediSearchResult<T>> + Send + 'static,
    T: IntoPy<PyObject>,
{
    let res = pyo3_asyncio::tokio::future_into_py(py, async { fut.await.map_err(Into::into) })
        .map(Into::into)?;
    Ok(res)
}

/// This class is used to transfer
/// data between python and rust.
///
