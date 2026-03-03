use hyper::StatusCode;
use wassel_http::{IntoResponse, Response};
use wassel_plugin_component::PluginHandleError;

#[derive(Debug, thiserror::Error)]
pub enum ServeError {
    #[error("WASI runtime error: {0}")]
    PluginError(#[from] PluginHandleError),
}

impl IntoResponse for ServeError {
    fn into_response(self) -> Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
