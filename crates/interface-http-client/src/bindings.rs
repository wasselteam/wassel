pub use wassel::http_client;

wasmtime::component::bindgen!({
    path: "wit",
    world: "platform-http-client",
    with: {
        "wasi:http": wasmtime_wasi_http::p2::bindings::http,
    },
    imports: { default: async, },
    exports: { default: async, },
});
