use crate::connection;

pub use wassel::postgres;

wasmtime::component::bindgen!({
    path: "wit",
    world: "platform-postgres",
    with: {
        "wassel:postgres/postgres.connection": connection::PgConnection,
        "wassel:postgres/postgres.connection-config": connection::PgConnectionConfig,
    },
    imports: {
        "wassel:postgres/postgres": async | trappable, default: async
    },
    exports: {
        default: async
    },
    trappable_error_type: {
        "wassel:postgres/postgres.error" => wassel::postgres::postgres::Error,
    }
});
