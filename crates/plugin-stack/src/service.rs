use std::pin::Pin;

use http_body_util::BodyExt;
use hyper::{Request, StatusCode, body::Incoming, service::Service};
use tracing::{error, trace};
use wassel_http::{Body, Error, IntoResponse as _, Response};

use crate::Stack;

use crate::errors::ServeError;

impl Service<Request<Incoming>> for Stack {
    type Response = Response;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let s = self.clone();
        Box::pin(async move { Ok(handle_request(s, req).await.into_response()) })
    }
}

async fn handle_request(s: Stack, req: Request<Incoming>) -> Result<Response, ServeError> {
    let plugin = match s.get_plugin(req.uri().path()).await {
        Ok(Some(p)) => p,
        Ok(None) => {
            trace!("No plugin found for {}", req.uri().path());
            return Ok(StatusCode::NOT_FOUND.into_response());
        }
        Err(e) => {
            error!("Could not get plugin for {}: {:#}", req.uri().path(), e);
            return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response());
        }
    };

    let response = plugin.handle(req).await.map_err(ServeError::PluginError)?;
    let (mut parts, body) = response.into_parts();
    parts.headers.insert(
        "x-wassel-plugin",
        plugin
            .id()
            .parse()
            .expect("Plugin ID should not be invalid header value"),
    );
    let response = Response::from_parts(parts, Body::new(body.map_err(Error::new)));
    Ok(response)
}
