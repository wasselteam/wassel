use std::time::Duration;

use http_body_util::{BodyExt as _, combinators::UnsyncBoxBody};
use wasmtime::component::{HasData, Resource, ResourceTable};
use wasmtime_wasi_http::{FieldMap, p2::body::HostIncomingBody};

use crate::{
    bindings::http_client::http_client::{self, ErrorCode, IncomingResponse, OutgoingRequest},
    body::StreamBody,
    reqwest::{convert_reqwest_error_to_error_code, convert_wasi_method_to_reqwest_method},
};

pub struct HttpClient;

impl HasData for HttpClient {
    type Data<'a> = HttpClientCtxView<'a>;
}

pub trait HttpClientView {
    fn http_client(&mut self) -> HttpClientCtxView<'_>;
}

pub struct HttpClientCtxView<'a> {
    pub table: &'a mut ResourceTable,
    pub client: &'a reqwest::Client,
}

impl http_client::Host for HttpClientCtxView<'_> {
    async fn send(
        &mut self,
        url: http_client::Url,
        req: Resource<OutgoingRequest>,
    ) -> Result<Resource<IncomingResponse>, ErrorCode> {
        let req = self.table.get_mut(&req).map_err(|e| {
            ErrorCode::InternalError(Some(format!(
                "Could not get OutgoingRequest resource: {e:?}"
            )))
        })?;

        let method = convert_wasi_method_to_reqwest_method(&req.method)
            .map_err(|_| ErrorCode::HttpRequestMethodInvalid)?;

        let mut request = self
            .client
            .request(method, url)
            .headers(req.headers.clone().into());

        if let Some(body) = req.body.take() {
            let body = reqwest::Body::wrap_stream(body.into_data_stream());
            request = request.body(body);
        }

        let response = request
            .send()
            .await
            .map_err(convert_reqwest_error_to_error_code)?;

        let status = response.status().into();
        let headers = response.headers().to_owned();

        let body_stream = Box::pin(response.bytes_stream());

        let hyper_body = UnsyncBoxBody::new(StreamBody::new(body_stream));
        let incoming_body = HostIncomingBody::new(hyper_body, Duration::from_secs(5));

        let response = IncomingResponse {
            status,
            headers: FieldMap::new_immutable(headers),
            body: Some(incoming_body),
        };

        let resource = self.table.push(response).map_err(|e| {
            ErrorCode::InternalError(Some(format!(
                "Could not create IncomingResponse resource: {e:?}"
            )))
        })?;

        Ok(resource)
    }
}
