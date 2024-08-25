use fred::clients::RedisPool;

#[pyclass(frozen, weakref)]
#[derive(Clone)]
pub struct RediSearchTable {
    pool: &RedisPool,
    name: String,
}
