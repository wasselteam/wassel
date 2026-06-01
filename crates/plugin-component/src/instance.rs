use std::{ops::DerefMut as _, str::FromStr};

use http::{Uri, uri::PathAndQuery};
use hyper::{Request, Response, body::Incoming};
use tokio::sync::{Mutex, MutexGuard};
use wasmtime::{Store, component::Instance};
use wasmtime_wasi_http::{
    p2::WasiHttpView as _, p2::bindings::http::types::Scheme, p2::body::HyperOutgoingBody,
};

use crate::{errors::PluginHandleError, state::PluginState};

pub struct PluginInstance {
    id: String,
    instance: Instance,
    store: Store<PluginState>,
    endpoint: String,
}

impl PluginInstance {
    pub fn new(
        id: String,
        instance: Instance,
        store: Store<PluginState>,
        endpoint: String,
    ) -> Self {
        Self {
            id,
            instance,
            store,
            endpoint,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub async fn handle(
        mut self,
        mut req: Request<Incoming>,
    ) -> Result<Response<HyperOutgoingBody>, PluginHandleError> {
        let (sender, reciever) = tokio::sync::oneshot::channel();

        let mut parts = req.uri().clone().into_parts();
        let paq = parts
            .path_and_query
            .expect("Path and query should be present in request");
        let paq = paq
            .as_str()
            .strip_prefix(&self.endpoint)
            .expect("URI must start with the plugin prefix");
        let paq = "/".to_owned() + paq;
        parts.path_and_query = Some(
            PathAndQuery::from_str(&paq)
                .expect("Parts and query should still be valid after stripping prefix"),
        );
        *req.uri_mut() =
            Uri::from_parts(parts).expect("URI should still be valid after stripping prefix");

        let req = self
            .store
            .data_mut()
            .http()
            .new_incoming_request(Scheme::Http, req)
            .map_err(PluginHandleError::CreateResource)?;

        let out = self
            .store
            .data_mut()
            .http()
            .new_response_outparam(sender)
            .map_err(PluginHandleError::CreateResource)?;

        let task = tokio::spawn(async move {
            let mut store = self.store;
            let proxy = wassel_world::HttpPlugin::new(&mut store, &self.instance)
                .map_err(PluginHandleError::Guest)?;

            proxy
                .wassel_foundation_http_handler()
                .call_handle_request(&mut store, req, out)
                .await
                .map_err(PluginHandleError::CallingHandleMethod)?;

            Ok::<(), PluginHandleError>(())
        });

        match reciever.await {
            Ok(Ok(response)) => Ok(response),
            Ok(Err(error)) => Err(error.into()),
            Err(_) => match task.await {
                Ok(Ok(())) => Err(PluginHandleError::ResponseOutparamWasNotSet),
                Ok(Err(error)) => Err(error),
                Err(error) => Err(error.into()),
            },
        }
    }
}
