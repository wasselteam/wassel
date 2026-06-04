use crate::bindings::redis::RedisArgument;
use crate::bindings::redis::RedisValue;

pub struct LazyRedisValue {
    pub value: RedisValue,
}

impl LazyRedisValue {
    pub fn new(value: RedisValue) -> Self {
        Self { value }
    }
}

pub struct RedisConnectionConfig {
    pub url: String,
}

pub struct RedisConnection {
    client: redis::Client,
}

impl RedisConnection {
    pub fn open(config: &RedisConnectionConfig) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(config.url.clone())?;
        Ok(Self { client })
    }

    pub async fn execute(
        &self,
        command: &str,
        args: &[RedisArgument],
    ) -> Result<redis::Value, redis::RedisError> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let mut cmd = redis::cmd(command);
        for arg in args {
            match arg {
                RedisArgument::I64(num) => cmd.arg(num),
                RedisArgument::Str(str) => cmd.arg(str),
            };
        }

        cmd.query_async(&mut conn).await
    }
}
