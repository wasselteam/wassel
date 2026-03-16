#![allow(unused)]

use std::{
    fmt::Display,
    pin::Pin,
    task::{Poll, ready},
};

use bytes::Bytes;
use futures::{Stream, TryStream};
use http_body_util::{BodyExt, combinators::UnsyncBoxBody};
use hyper::body::Frame;
use pin_project::pin_project;
use sync_wrapper::SyncWrapper;
use tokio::sync::mpsc;

pub type BoxError = Box<dyn std::error::Error + Send + Sync>;

#[must_use]
#[derive(Debug)]
pub struct Error(BoxError);

impl Error {
    pub fn new(error: impl Into<BoxError>) -> Self {
        Self(error.into())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.0.source()
    }
}

pub type BoxBody = UnsyncBoxBody<Bytes, Error>;

#[must_use]
pub struct Body(BoxBody);

impl Body {
    pub fn new<B>(body: B) -> Self
    where
        B: hyper::body::Body<Data = Bytes> + Send + 'static,
        B::Error: Into<BoxError>,
    {
        Self(body.map_err(Error::new).boxed_unsync())
    }

    pub fn empty() -> Self {
        Self::new(http_body_util::Empty::new())
    }

    pub fn from_stream<S>(s: S) -> Self
    where
        S: TryStream + Send + 'static,
        S::Ok: Into<Bytes>,
        S::Error: Into<BoxError>,
    {
        Self::new(StreamBody {
            stream: SyncWrapper::new(s),
        })
    }

    pub fn from_channel<B: Into<Bytes> + Send + 'static>(r: mpsc::Receiver<B>) -> Self {
        Self::new(ChannelBody::new(r))
    }
}

impl hyper::body::Body for Body {
    type Data = Bytes;
    type Error = Error;

    #[inline]
    fn poll_frame(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Result<hyper::body::Frame<Self::Data>, Self::Error>>> {
        Pin::new(&mut self.0).poll_frame(cx)
    }

    fn is_end_stream(&self) -> bool {
        self.0.is_end_stream()
    }

    fn size_hint(&self) -> hyper::body::SizeHint {
        self.0.size_hint()
    }
}

#[pin_project]
pub struct StreamBody<S> {
    #[pin]
    stream: SyncWrapper<S>,
}

impl<S> hyper::body::Body for StreamBody<S>
where
    S: TryStream,
    S::Ok: Into<Bytes>,
    S::Error: Into<BoxError>,
{
    type Data = Bytes;
    type Error = Error;

    fn poll_frame(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        let mut s = self.project().stream.get_pin_mut();
        match ready!(s.try_poll_next(cx)) {
            Some(Ok(data)) => Poll::Ready(Some(Ok(Frame::data(data.into())))),
            Some(Err(error)) => Poll::Ready(Some(Err(Error::new(error)))),
            None => Poll::Ready(None),
        }
    }
}

pub struct ChannelBody<B: Into<Bytes>> {
    chan: mpsc::Receiver<B>,
}

impl<B: Into<Bytes>> ChannelBody<B> {
    pub fn new(chan: mpsc::Receiver<B>) -> Self {
        Self { chan }
    }
}

impl<B: Into<Bytes>> hyper::body::Body for ChannelBody<B> {
    type Data = Bytes;
    type Error = Error;

    fn poll_frame(
        self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> Poll<Option<Result<Frame<Self::Data>, Self::Error>>> {
        match ready!(self.get_mut().chan.poll_recv(cx)) {
            Some(msg) => Poll::Ready(Some(Ok(Frame::data(msg.into())))),
            None => Poll::Ready(None),
        }
    }
}

macro_rules! body_from_impl {
    ($ty:ty) => {
        impl From<$ty> for Body {
            fn from(buf: $ty) -> Self {
                Self::new(http_body_util::Full::from(buf))
            }
        }
    };
}

body_from_impl!(Bytes);

body_from_impl!(&'static [u8]);
body_from_impl!(Vec<u8>);
body_from_impl!(std::borrow::Cow<'static, [u8]>);

body_from_impl!(&'static str);
body_from_impl!(String);
body_from_impl!(std::borrow::Cow<'static, str>);
