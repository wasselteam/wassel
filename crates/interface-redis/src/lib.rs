pub mod bindings;
mod convert;
mod host;
mod types;

pub use host::{Redis, RedisCtxView, RedisView};
use wasmtime::component::Linker;

pub fn add_to_linker<T: RedisView + Send + 'static>(
    linker: &mut Linker<T>,
) -> wasmtime::Result<()> {
    bindings::wassel::redis::redis::add_to_linker::<T, Redis>(linker, T::redis)
}
