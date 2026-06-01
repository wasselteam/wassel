use tokio::sync::oneshot::error::RecvError;
use wasmtime_wasi_http::p2::bindings::http::types::ErrorCode;

#[derive(Debug, thiserror::Error)]
pub enum PluginHandleError {
    #[error("Could not create resource: {0}")]
    CreateResource(wasmtime::Error),

    #[error("Error occured when trying to call handle method: {0}")]
    CallingHandleMethod(wasmtime::Error),

    #[error("Could not recieve response from plugin: {0}")]
    RecieveResponse(#[from] RecvError),

    #[error("Plugin returned error code: {0}")]
    ErrorCode(#[from] ErrorCode),

    #[error("Could not create component guest")]
    Guest(wasmtime::Error),

    #[error("Guest never invoked `response-outparam::set` method")]
    ResponseOutparamWasNotSet,

    #[error("Could not join tokio task: {0}")]
    TaskJoin(#[from] tokio::task::JoinError),
}
