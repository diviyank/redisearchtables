use fred::clients::RedisPool;

#[pyclass(frozen, weakref)]
#[derive(Clone)]
pub struct RediSearchPool {
    _pool: RedisPool,
}
#[pymethods]
impl RedisSearchPool {
    #[new]
    #[pyo3(signature = (
        *,
    ))]
    #[allow(clippy::too_many_arguments)]
    pub fn py_new(
    ) -> Self {
        RedisSearchPool {
            _pool:RedisPool::new()
        }
    }
