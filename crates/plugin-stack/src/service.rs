use std::pin::Pin;

use http_body_util::BodyExt;
use hyper::{Request, StatusCode, body::Incoming, service::Service};
use tokio::time;
use tracing::{Instrument, Level, error, info, span, trace};
use wassel_http::{Body, Error, IntoResponse as _, Response};

use crate::Stack;

use crate::errors::ServeError;

impl Service<Request<Incoming>> for Stack {
    type Response = Response;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let s = self.clone();
        

        (Box::pin(
            async move {
                let method = req.method().as_str().to_owned();
                let path = req.uri().path().to_owned();

                let begin = time::Instant::now();
                let (response, id) = handle_request(s, req).await;
                let end = time::Instant::now();

                let status = response.status().as_u16();
                let delay = (end - begin).as_secs_f64();

                info!(
                    method,
                    path,
                    status,
                    delay,
                    plugin_id = id,
                    "{method} {path}: {status} ({delay:04}ms)",
                    delay = delay * 1000.0
                );

                Ok(response)
            }
            .instrument(span!(Level::DEBUG, "handling request")),
        )) as _
    }
}

async fn handle_request(s: Stack, req: Request<Incoming>) -> (Response, Option<String>) {
    let plugin = match s.get_plugin(req.uri().path()).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            trace!("No plugin found for {}", req.uri().path());
            return (StatusCode::NOT_FOUND.into_response(), None);
        }
        Err(e) => {
            error!("Could not get plugin for {}: {:#}", req.uri().path(), e);
            return (StatusCode::INTERNAL_SERVER_ERROR.into_response(), None);
        }
    };

    let id = plugin.id().to_owned();

    let response = match plugin.handle(req).await {
        Ok(response) => response,
        Err(e) => return (ServeError::PluginError(e).into_response(), Some(id)),
    };

    let (mut parts, body) = response.into_parts();
    parts.headers.insert(
        "x-wassel-plugin",
        plugin
            .id()
            .parse()
            .expect("Plugin ID should not be invalid header value"),
    );

    (
        Response::from_parts(parts, Body::new(body.map_err(Error::new))),
        Some(id),
    )
}
