use std::{fs, io, path::Path, pin::Pin};

use http::{HeaderMap, HeaderValue, StatusCode, header::CONTENT_TYPE};
use hyper::{Request, body::Incoming, service::Service};

use wassel_http::{Error, IntoResponse, Response};
use wassel_plugin_stack::Stack;

use crate::stats::Stats;

#[derive(Clone)]
pub struct AdminService {
    stack: Stack,
}

impl AdminService {
    pub fn new(stack: Stack) -> Self {
        Self { stack }
    }
}

impl Service<Request<Incoming>> for AdminService {
    type Response = Response;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let s = self.clone();

        let future = async move {
            let mut uri = req.uri().path();

            if uri == "/stats" {
                let mut plugins = s.stack.plugin_list();
                plugins.sort_by(|a, b| a.name.cmp(&b.name));
                let bytes = serde_json::to_vec(&Stats::new(plugins)).unwrap();
                let response = (
                    StatusCode::OK,
                    HeaderMap::from_iter([(
                        CONTENT_TYPE,
                        HeaderValue::from_static("application/json"),
                    )]),
                    bytes,
                )
                    .into_response();
                return Ok(response);
            }

            if uri.starts_with("/") {
                uri = &uri[1..];
            }

            let path = if uri.is_empty() {
                Path::new("assets/index.html")
            } else {
                &Path::new("assets").join(uri)
            };

            let content_type = match path
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
            {
                "html" => Some("text/html"),
                "js" => Some("text/javascript"),
                _ => None,
            };

            let data = match fs::read(path) {
                Ok(data) => data,
                Err(e) => match e.kind() {
                    io::ErrorKind::NotFound | io::ErrorKind::PermissionDenied => {
                        return Ok(StatusCode::NOT_FOUND.into_response());
                    }
                    _ => return Ok(StatusCode::INTERNAL_SERVER_ERROR.into_response()),
                },
            };

            let mut response = data.into_response();
            if let Some(content_type) = content_type {
                response
                    .headers_mut()
                    .insert(CONTENT_TYPE, HeaderValue::from_static(content_type));
            }

            Ok(response)
        };

        Box::pin(future)
    }
}
