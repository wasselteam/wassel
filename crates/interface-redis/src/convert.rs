use redis::ToRedisArgs;
use wasmtime::component::Resource;
use wasmtime_wasi::{ResourceTable, ResourceTableError};

use crate::{
    bindings::redis::{
        Attribute, Error, Push, PushKind, RedisArgument, RedisValue, VerbatimFormat, VerbatimString,
    },
    types::LazyRedisValue,
};

impl ToRedisArgs for RedisArgument {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite,
    {
        match self {
            RedisArgument::I64(v) => out.write_arg_fmt(v),
            RedisArgument::Str(v) => out.write_arg_fmt(v),
        }
    }
}

pub fn redis_value_to_wasm(
    table: &mut ResourceTable,
    v: redis::Value,
) -> Result<RedisValue, Error> {
    let val = match v {
        redis::Value::Nil => RedisValue::Nil,
        redis::Value::Int(v) => RedisValue::Int(v),
        redis::Value::BulkString(v) => RedisValue::BulkString(v),
        redis::Value::Array(v) => RedisValue::Array(convert_vec_to_wasm(table, v)?),
        redis::Value::SimpleString(v) => RedisValue::SimpleString(v),
        redis::Value::Okay => RedisValue::Okay,
        redis::Value::Map(items) => RedisValue::Map(convert_map_to_wasm(table, items)?),
        redis::Value::Attribute { data, attributes } => RedisValue::Attribute(Attribute {
            data: convert_value_to_lazy(table, *data)?,
            attributes: convert_map_to_wasm(table, attributes)?,
        }),
        redis::Value::Set(values) => RedisValue::Set(convert_vec_to_wasm(table, values)?),
        redis::Value::Double(v) => RedisValue::Double(v),
        redis::Value::Boolean(v) => RedisValue::Boolean(v),
        redis::Value::VerbatimString { format, text } => {
            RedisValue::VerbatimString(VerbatimString {
                format: match format {
                    redis::VerbatimFormat::Unknown(f) => VerbatimFormat::Unknown(f),
                    redis::VerbatimFormat::Markdown => VerbatimFormat::Markdown,
                    redis::VerbatimFormat::Text => VerbatimFormat::Text,
                    other => {
                        return Err(Error::TypeError(format!(
                            "Unknown verbatim format `{other:?}`"
                        )));
                    }
                },
                text,
            })
        }
        redis::Value::BigNumber(big_int) => RedisValue::BigNumber(big_int.to_string()),
        redis::Value::Push { kind, data } => RedisValue::Push(Push {
            kind: match kind {
                redis::PushKind::Disconnection => PushKind::Disconnection,
                redis::PushKind::Other(k) => PushKind::Other(k),
                redis::PushKind::Invalidate => PushKind::Invalidate,
                redis::PushKind::Message => PushKind::Message,
                redis::PushKind::PMessage => PushKind::Pmessage,
                redis::PushKind::SMessage => PushKind::Smessage,
                redis::PushKind::Unsubscribe => PushKind::Unsubscribe,
                redis::PushKind::PUnsubscribe => PushKind::Punsubscribe,
                redis::PushKind::SUnsubscribe => PushKind::Sunsubscribe,
                redis::PushKind::Subscribe => PushKind::Subscribe,
                redis::PushKind::PSubscribe => PushKind::Psubscribe,
                redis::PushKind::SSubscribe => PushKind::Ssubscribe,
                other => {
                    return Err(Error::TypeError(format!("Unknown push kind `{other:?}`")));
                }
            },
            data: convert_vec_to_wasm(table, data)?,
        }),
        redis::Value::ServerError(server_error) => {
            return Err(Error::ServerError(server_error.to_string()));
        }
        other => {
            return Err(Error::TypeError(format!(
                "Could not convert value `{other:?}`"
            )));
        }
    };

    Ok(val)
}

fn convert_value_to_lazy(
    table: &mut ResourceTable,
    value: redis::Value,
) -> Result<Resource<LazyRedisValue>, wasmtime::Error> {
    let value = redis_value_to_wasm(table, value)?;
    let resource = table.push(LazyRedisValue::new(value))?;
    Ok(resource)
}

fn convert_vec_to_wasm(
    table: &mut ResourceTable,
    values: Vec<redis::Value>,
) -> Result<Vec<Resource<LazyRedisValue>>, wasmtime::Error> {
    let mut result = Vec::new();
    for value in values {
        result.push(convert_value_to_lazy(table, value)?);
    }
    Ok(result)
}

#[allow(clippy::type_complexity)]
fn convert_map_to_wasm(
    table: &mut ResourceTable,
    values: Vec<(redis::Value, redis::Value)>,
) -> Result<Vec<(Resource<LazyRedisValue>, Resource<LazyRedisValue>)>, wasmtime::Error> {
    let mut result = Vec::new();
    for (left, right) in values {
        result.push((
            convert_value_to_lazy(table, left)?,
            convert_value_to_lazy(table, right)?,
        ));
    }
    Ok(result)
}

impl From<wasmtime::Error> for Error {
    fn from(value: wasmtime::Error) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<ResourceTableError> for Error {
    fn from(value: ResourceTableError) -> Self {
        Self::Other(value.to_string())
    }
}

impl From<redis::RedisError> for Error {
    fn from(value: redis::RedisError) -> Self {
        Self::ServerError(value.to_string())
    }
}
