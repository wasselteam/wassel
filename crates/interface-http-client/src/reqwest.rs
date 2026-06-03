use http::method::InvalidMethod;
use reqwest::Method;
use wasmtime_wasi_http::p2::bindings::http::types::Method as WasiMethod;

use crate::bindings::http_client::http_client::ErrorCode;

pub fn convert_wasi_method_to_reqwest_method(method: &WasiMethod) -> Result<Method, InvalidMethod> {
    let method = match method {
        WasiMethod::Get => Method::GET,
        WasiMethod::Head => Method::HEAD,
        WasiMethod::Post => Method::POST,
        WasiMethod::Put => Method::PUT,
        WasiMethod::Delete => Method::DELETE,
        WasiMethod::Connect => Method::CONNECT,
        WasiMethod::Options => Method::OPTIONS,
        WasiMethod::Trace => Method::TRACE,
        WasiMethod::Patch => Method::PATCH,
        WasiMethod::Other(m) => Method::from_bytes(m.as_bytes())?,
    };

    Ok(method)
}

pub fn convert_reqwest_error_to_error_code(e: reqwest::Error) -> ErrorCode {
    ErrorCode::InternalError(Some(format!("Reqwest error: {e:?}")))
}
