pub mod bindings;
mod body;
mod host;
mod reqwest;

use wasmtime::component::Linker;

pub use host::{HttpClient, HttpClientCtxView, HttpClientView};

pub fn add_to_linker<T: HttpClientView + Send + 'static>(
    linker: &mut Linker<T>,
) -> wasmtime::Result<()> {
    bindings::http_client::http_client::add_to_linker::<T, HttpClient>(linker, T::http_client)
}
