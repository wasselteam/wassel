// use crate::connection;

pub use wassel::redis::redis;

wasmtime::component::bindgen!({
    path: "wit",
    world: "platform-redis",
    with: {
         "wassel:redis/redis.lazy-redis-value": crate::types::LazyRedisValue,
         "wassel:redis/redis.connection": crate::types::RedisConnection,
         "wassel:redis/redis.connection-config": crate::types::RedisConnectionConfig,
    },
    imports: {
        "wassel:redis/redis": async | trappable, default: async
    },
    exports: {
        default: async
    },
    trappable_error_type: {
        "wassel:redis/redis.error" => redis::Error,
    }
});
