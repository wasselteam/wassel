use std::{collections::HashMap, path::Path};

use lazy_static::lazy_static;
use wasmtime_wasi::{
    DirPerms, FilePerms, ResourceTable, WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView,
};
use wasmtime_wasi_config::WasiConfigVariables;
use wasmtime_wasi_http::{
    WasiHttpCtx,
    p2::{WasiHttpCtxView, WasiHttpView},
};
use wassel_interface_http_client::{HttpClientCtxView, HttpClientView};
use wassel_interface_postgres::{PostgresCtxView, PostgresView};

use wasmtime::error::Context as _;
use wassel_interface_redis::{RedisCtxView, RedisView};
use wassel_world::{WasiConfigCtxView, WasiConfigView};

lazy_static! {
    static ref HTTP_CLIENT: reqwest::Client = reqwest::Client::new();
}

pub struct PluginState {
    ctx: WasiCtx,
    config_vars: WasiConfigVariables,
    table: ResourceTable,
    http_ctx: WasiHttpCtx,
}

impl PluginState {
    pub fn new(
        data_dir: impl AsRef<Path>,
        config: &HashMap<String, String>,
    ) -> anyhow::Result<Self> {
        let ctx = {
            let mut builder = WasiCtxBuilder::new();
            builder.inherit_stdout();
            builder.inherit_stderr();
            builder
                .preopened_dir(data_dir.as_ref(), ".", DirPerms::all(), FilePerms::all())
                .context(format!(
                    "Preopening data directory `{}`",
                    data_dir.as_ref().to_string_lossy()
                ))?;
            builder.build()
        };

        let s = Self {
            ctx,
            config_vars: WasiConfigVariables::from_iter(config),
            table: ResourceTable::new(),
            http_ctx: WasiHttpCtx::new(),
        };

        Ok(s)
    }

    pub fn config_vars(&self) -> &WasiConfigVariables {
        &self.config_vars
    }
}

impl WasiView for PluginState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.ctx,
            table: &mut self.table,
        }
    }
}

impl WasiHttpView for PluginState {
    fn http(&mut self) -> WasiHttpCtxView<'_> {
        WasiHttpCtxView {
            ctx: &mut self.http_ctx,
            table: &mut self.table,
            hooks: Default::default(),
        }
    }
}

impl WasiConfigView for PluginState {
    fn wasi_config(&mut self) -> WasiConfigCtxView<'_> {
        WasiConfigCtxView {
            variables: &self.config_vars,
        }
    }
}

impl PostgresView for PluginState {
    fn postgres(&mut self) -> PostgresCtxView<'_> {
        PostgresCtxView {
            table: &mut self.table,
        }
    }
}

impl HttpClientView for PluginState {
    fn http_client(&mut self) -> HttpClientCtxView<'_> {
        HttpClientCtxView {
            table: &mut self.table,
            client: &HTTP_CLIENT,
        }
    }
}

impl RedisView for PluginState {
    fn redis(&mut self) -> RedisCtxView<'_> {
        RedisCtxView {
            table: &mut self.table,
        }
    }
}
