use std::pin::Pin;

use futures_util::{Stream, TryStreamExt as _};
use hyper::body::Bytes;

use crate::{
    bindings::http_client::http_client::ErrorCode, reqwest::convert_reqwest_error_to_error_code,
};

pub struct StreamBody<S> {
    stream: S,
}

impl<S> StreamBody<S> {
    pub fn new(stream: S) -> Self {
        StreamBody { stream }
    }
}

impl<S> hyper::body::Body for StreamBody<Pin<Box<S>>>
where
    S: Stream<Item = reqwest::Result<Bytes>>,
{
    type Data = Bytes;

    type Error = ErrorCode;

    fn poll_frame(
        self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Result<hyper::body::Frame<Self::Data>, Self::Error>>> {
        match self.get_mut().stream.try_poll_next_unpin(cx) {
            std::task::Poll::Ready(v) => match v {
                Some(result) => std::task::Poll::Ready(Some(match result {
                    Ok(bytes) => Ok(hyper::body::Frame::data(bytes)),
                    Err(e) => Err(convert_reqwest_error_to_error_code(e)),
                })),
                None => std::task::Poll::Ready(None),
            },
            std::task::Poll::Pending => std::task::Poll::Pending,
        }
    }

    fn size_hint(&self) -> hyper::body::SizeHint {
        let (lower, upper) = self.stream.size_hint();
        let mut hint = hyper::body::SizeHint::new();
        hint.set_lower(lower as u64);
        if let Some(upper) = upper {
            hint.set_upper(upper as u64);
        }
        hint
    }
}
