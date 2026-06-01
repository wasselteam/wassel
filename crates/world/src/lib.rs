wasmtime::component::bindgen!({
    path: "../../wit",
    world: "http-plugin",
    with: {
        "wasi:http": wasmtime_wasi_http::p2::bindings::http,
        "wassel:postgres": wassel_interface_postgres::bindings::postgres,
    },
    imports: { default: async, },
    exports: { default: async, },
});
