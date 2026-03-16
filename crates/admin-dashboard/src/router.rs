use std::{pin::Pin, sync::Arc};

use http::{Request, StatusCode};
use hyper::body::Incoming;
use tracing::debug;
use wassel_http::{IntoResponse, Response};

#[derive(Clone)]
pub struct Router<S: Clone + Send + 'static> {
    state: S,
    inner: Arc<matchit::Router<BoxHandler<S>>>,
}

pub struct BoxHandler<S: Clone + 'static>(Box<dyn Handler<S>>);

impl<S: Clone + 'static> std::ops::Deref for BoxHandler<S> {
    type Target = Box<dyn Handler<S>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<S: Clone + Send + 'static> BoxHandler<S> {
    pub fn new(handler: impl Handler<S>) -> Self {
        Self(Box::new(handler))
    }
}

impl<S: Clone + Send + 'static> Clone for BoxHandler<S> {
    fn clone(&self) -> Self {
        Self(self.0.clone_box())
    }
}

impl<S: Clone + Send + 'static> Router<S> {
    pub fn new(state: S) -> Self {
        Self {
            state,
            inner: Arc::new(matchit::Router::default()),
        }
    }

    pub fn route(self, path: impl Into<String>, handler: impl Handler<S>) -> Self {
        let state = self.state;
        let mut inner = Arc::unwrap_or_clone(self.inner);

        inner
            .insert(path, BoxHandler::new(handler))
            .expect("Could not insert route");

        Self {
            state,
            inner: Arc::new(inner),
        }
    }

    pub async fn handle(&self, req: Request<Incoming>) -> Response {
        debug!("Matching path: `{}`", req.uri().path());

        let Ok(matched) = self.inner.at(req.uri().path()) else {
            return StatusCode::NOT_FOUND.into_response();
        };

        let handler = matched.value;
        handler
            .clone_box()
            .call(self.state.clone(), req)
            .await
            .into_response()
    }
}

pub trait Handler<S: Clone + Send + 'static>: Send + Sync + 'static {
    fn call(
        self: Box<Self>,
        state: S,
        req: Request<Incoming>,
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>>;

    fn clone_box(&self) -> Box<dyn Handler<S>>;
}

impl<F, Fut, I, S> Handler<S> for F
where
    F: FnOnce(S, Request<Incoming>) -> Fut + Clone + Send + Sync + 'static,
    Fut: Future<Output = I> + Send + 'static,
    I: IntoResponse,
    S: Clone + Send + 'static,
{
    fn call(
        self: Box<Self>,
        state: S,
        req: Request<Incoming>,
    ) -> Pin<Box<dyn Future<Output = Response> + Send + 'static>> {
        Box::pin(async move { self(state, req).await.into_response() })
    }

    fn clone_box(&self) -> Box<dyn Handler<S>> {
        Box::new(self.clone())
    }
}
