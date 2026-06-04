use wasmtime::component::{HasData, Resource};
use wasmtime_wasi::ResourceTable;

use crate::{
    bindings::redis::{
        Error, Host, HostConnection, HostConnectionConfig, HostLazyRedisValue, RedisArgument,
        RedisValue,
    },
    convert::redis_value_to_wasm,
    types::{LazyRedisValue, RedisConnection, RedisConnectionConfig},
};

pub struct Redis;

impl HasData for Redis {
    type Data<'a> = RedisCtxView<'a>;
}

pub trait RedisView {
    fn redis(&mut self) -> RedisCtxView<'_>;
}

pub struct RedisCtxView<'a> {
    pub table: &'a mut ResourceTable,
}

impl<'a> Host for RedisCtxView<'a> {
    fn convert_error(&mut self, err: Error) -> wasmtime::Result<Error> {
        Ok(err)
    }
}

impl<'a> HostLazyRedisValue for RedisCtxView<'a> {
    async fn new(&mut self, value: RedisValue) -> wasmtime::Result<Resource<LazyRedisValue>> {
        let resource = self.table.push(LazyRedisValue::new(value))?;
        Ok(resource)
    }

    async fn drop(&mut self, rep: Resource<LazyRedisValue>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }

    async fn get(&mut self, self_: Resource<LazyRedisValue>) -> wasmtime::Result<RedisValue> {
        let value = self.table.delete(self_)?;
        Ok(value.value)
    }
}

impl<'a> HostConnection for RedisCtxView<'a> {
    async fn open(
        &mut self,
        config: Resource<RedisConnectionConfig>,
    ) -> Result<Resource<RedisConnection>, Error> {
        let config = self.table.get(&config)?;
        let conn = RedisConnection::open(config)?;
        let resource = self.table.push(conn)?;
        Ok(resource)
    }

    async fn execute(
        &mut self,
        self_: Resource<RedisConnection>,
        command: String,
        arguments: Vec<RedisArgument>,
    ) -> Result<RedisValue, Error> {
        let conn = self.table.get(&self_)?;
        let result = conn.execute(&command, &arguments).await?;
        let value = redis_value_to_wasm(self.table, result)?;
        Ok(value)
    }

    async fn drop(&mut self, rep: Resource<RedisConnection>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}

impl<'a> HostConnectionConfig for RedisCtxView<'a> {
    async fn new(
        &mut self,
        connection_string: String,
    ) -> wasmtime::Result<Resource<RedisConnectionConfig>> {
        let config = RedisConnectionConfig {
            url: connection_string,
        };
        let resource = self.table.push(config)?;
        Ok(resource)
    }

    async fn drop(&mut self, rep: Resource<RedisConnectionConfig>) -> wasmtime::Result<()> {
        self.table.delete(rep)?;
        Ok(())
    }
}
