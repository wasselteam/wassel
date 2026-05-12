mod postgres;

wasmtime::component::bindgen!({
    path: "../../wit",
    world: "http-plugin",
    with: {
        "wasi:http": wasmtime_wasi_http::p2::bindings::http,
        "wassel:foundation/postgres.connection": postgres::PgConnection,
        "wassel:foundation/postgres.connection-config": postgres::PgConnectionConfig,
    },
    imports: {
        "wassel:foundation/postgres": async | trappable,
        default: async,
    },
    exports: { default: async },

    trappable_error_type: {
        "wassel:foundation/postgres.error" => wassel::foundation::postgres::Error,
    }
});
