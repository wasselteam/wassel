pub mod bindings;
mod connection;
mod host;

pub use host::{Postgres, PostgresCtxView, PostgresView};
use wasmtime::component::Linker;

pub fn add_to_linker<T: PostgresView + Send + 'static>(
    linker: &mut Linker<T>,
) -> wasmtime::Result<()> {
    bindings::wassel::postgres::postgres::add_to_linker::<T, Postgres>(linker, T::postgres)
}
