use std::path::PathBuf;

use wasmtime::{
    Engine,
    component::{Component, InstancePre},
    error::Context as _,
};

use crate::{instance::PluginInstance, meta::PluginMeta, state::PluginState};

pub struct PluginImage {
    pre: InstancePre<PluginState>,
    meta: PluginMeta,
    data_dir: PathBuf,
}

impl PluginImage {
    pub async fn load(
        engine: &Engine,
        bytes: &[u8],
        meta: PluginMeta,
        data_dir: impl Into<PathBuf>,
    ) -> anyhow::Result<Self> {
        let component = Component::new(engine, bytes).context("Creating WASM component")?;

        let mut linker = wasmtime::component::Linker::<PluginState>::new(engine);
        wassel_world::add_to_linker(&mut linker).context("Could not add wassel world to linker")?;

        let export = "wassel:foundation/http-handler";
        if component.get_export(None, export).is_none() {
            anyhow::bail!("There is no '{export}' export");
        }

        let pre = linker
            .instantiate_pre(&component)
            .context("Pre-instantiating plugin")?;

        let image = Self {
            pre,
            meta,
            data_dir: data_dir.into(),
        };

        Ok(image)
    }

    pub async fn instantiate(&self, engine: &Engine) -> anyhow::Result<PluginInstance> {
        let mut store = wasmtime::Store::new(
            engine,
            PluginState::new(&self.data_dir, &self.meta.variables)?,
        );

        let instance = self.pre.instantiate_async(&mut store).await?;

        Ok(PluginInstance::new(
            self.id().to_owned(),
            instance,
            store,
            self.meta.endpoint.clone(),
        ))
    }

    pub fn id(&self) -> &str {
        &self.meta.id
    }

    pub fn meta(&self) -> &PluginMeta {
        &self.meta
    }
}
