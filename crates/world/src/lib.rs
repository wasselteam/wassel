use wasmtime::component::{Linker, bindgen};
use wasmtime_wasi::WasiView;
use wasmtime_wasi_config::{WasiConfig, WasiConfigVariables};
use wasmtime_wasi_http::p2::WasiHttpView;
use wassel_interface_http_client::HttpClientView;
use wassel_interface_postgres::PostgresView;
use wassel_interface_redis::RedisView;

bindgen!({
    path: "../../wit",
    world: "http-plugin",
    with: {
        "wasi:http": wasmtime_wasi_http::p2::bindings::http,
        "wassel:postgres": wassel_interface_postgres::bindings::postgres,
        "wassel:http-client": wassel_interface_http_client::bindings::http_client,
    },
    imports: { default: async, },
    exports: { default: async, },
});

pub fn add_to_linker<
    T: WasiView
        + WasiHttpView
        + WasiConfigView
        + HttpClientView
        + PostgresView
        + RedisView
        + Send
        + 'static,
>(
    linker: &mut Linker<T>,
) -> wasmtime::Result<()> {
    wasmtime_wasi::p2::add_to_linker_async(linker)?;
    wasmtime_wasi_http::p2::add_only_http_to_linker_async(linker)?;
    wasmtime_wasi_config::add_to_linker(linker, |c| WasiConfig::from(c.wasi_config().variables))?;
    wassel_interface_http_client::add_to_linker(linker)?;
    wassel_interface_postgres::add_to_linker(linker)?;
    wassel_interface_redis::add_to_linker(linker)?;

    Ok(())
}

pub trait WasiConfigView {
    fn wasi_config(&mut self) -> WasiConfigCtxView<'_>;
}

pub struct WasiConfigCtxView<'a> {
    pub variables: &'a WasiConfigVariables,
}
