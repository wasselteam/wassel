use hyper::{HeaderMap, body::Body as _, header::CONTENT_LENGTH, http::StatusCode};

use crate::body::Body;

pub type Response = hyper::Response<Body>;

pub trait IntoResponse {
    fn into_response(self) -> Response;
}

impl IntoResponse for StatusCode {
    fn into_response(self) -> Response {
        let body = Body::empty();
        let mut resp = hyper::Response::new(body);
        *resp.status_mut() = self;
        resp
    }
}

impl IntoResponse for Response {
    fn into_response(self) -> Response {
        self
    }
}

impl<B> IntoResponse for B
where
    B: Into<Body>,
{
    fn into_response(self) -> Response {
        let b = self.into();
        let size = b.size_hint().exact();
        let mut response = Response::new(b);
        *response.status_mut() = StatusCode::OK;
        if let Some(size) = size {
            response.headers_mut().insert(CONTENT_LENGTH, size.into());
        }
        response
    }
}

impl IntoResponse for HeaderMap {
    fn into_response(self) -> Response {
        let mut response = Response::new(Body::empty());
        *response.headers_mut() = self;
        response
    }
}

impl<B> IntoResponse for (HeaderMap, B)
where
    B: Into<Body>,
{
    fn into_response(self) -> Response {
        let mut response = self.1.into_response();
        *response.headers_mut() = self.0;
        response
    }
}

impl<B> IntoResponse for (StatusCode, B)
where
    B: Into<Body>,
{
    fn into_response(self) -> Response {
        let mut response = self.1.into_response();
        *response.status_mut() = self.0;
        response
    }
}

impl IntoResponse for (StatusCode, HeaderMap) {
    fn into_response(self) -> Response {
        let mut response = self.0.into_response();
        *response.headers_mut() = self.1;
        response
    }
}

impl<B> IntoResponse for (StatusCode, HeaderMap, B)
where
    B: Into<Body>,
{
    fn into_response(self) -> Response {
        let mut response = (self.0, self.2).into_response();
        *response.headers_mut() = self.1;
        response
    }
}

impl<T, E> IntoResponse for Result<T, E>
where
    T: IntoResponse,
    E: IntoResponse,
{
    fn into_response(self) -> Response {
        match self {
            Ok(v) => v.into_response(),
            Err(e) => e.into_response(),
        }
    }
}
