use std::{
    pin::Pin,
    sync::{Arc, Mutex},
    time::Duration,
};

use http::{HeaderMap, HeaderValue, StatusCode, header::CONTENT_TYPE};
use hyper::{Request, body::Incoming, service::Service};

use include_dir::{Dir, include_dir};
use serde::Serialize;
use tokio::sync::mpsc;
use tracing::debug;
use wassel_http::{Body, Error, IntoResponse, Response};
use wassel_plugin_stack::Stack;

use crate::{
    router::Router,
    sse::SseMessage,
    stats::{PluginStats, SystemStats},
};

static WEB_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/web/build");

#[derive(Clone)]
pub struct AdminService {
    router: Router<Arc<State>>,
}

pub struct State {
    stack: Stack,
    info: Mutex<sysinfo::System>,
}

impl AdminService {
    pub fn new(stack: Stack) -> Self {
        let state = State {
            stack,
            info: Mutex::new(sysinfo::System::new_all()),
        };

        let router = Router::new(Arc::new(state))
            .route("/api/stats/system", handle_stats_system)
            .route("/api/stats/plugins", handle_stats_plugins)
            .route("/api/stats/logs", handle_stats_logs)
            .route("/api/stats/sse", handle_stats_sse)
            .route("/{*path}", handle_web)
            .route("/", handle_web);

        Self { router }
    }
}

impl Service<Request<Incoming>> for AdminService {
    type Response = Response;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn call(&self, req: Request<Incoming>) -> Self::Future {
        let s = self.clone();

        let future = async move {
            let response = s.router.handle(req).await;
            Ok(response)
        };

        Box::pin(future)
    }
}

struct Json<S: Serialize>(S);

impl<S: Serialize> IntoResponse for Json<S> {
    fn into_response(self) -> Response {
        let Ok(bytes) = serde_json::to_vec(&self.0) else {
            return StatusCode::INTERNAL_SERVER_ERROR.into_response();
        };

        (
            StatusCode::OK,
            HeaderMap::from_iter([(CONTENT_TYPE, HeaderValue::from_static("application/json"))]),
            bytes,
        )
            .into_response()
    }
}

async fn handle_stats_system(state: Arc<State>, _req: Request<Incoming>) -> impl IntoResponse {
    let stats = SystemStats::load(&mut state.info.lock().unwrap());
    Json(stats)
}

async fn handle_stats_plugins(state: Arc<State>, _req: Request<Incoming>) -> impl IntoResponse {
    let mut plugins: Vec<PluginStats> = state
        .stack
        .plugin_list()
        .into_iter()
        .map(From::from)
        .collect();
    plugins.sort_by(|a, b| a.name.cmp(&b.name));
    Json(plugins)
}

async fn handle_stats_logs(_state: Arc<State>, _req: Request<Incoming>) -> impl IntoResponse {
    StatusCode::NOT_IMPLEMENTED
}

async fn handle_stats_sse(state: Arc<State>, _req: Request<Incoming>) -> impl IntoResponse {
    let s = state.clone();
    let (sender, receiver) = mpsc::channel::<SseMessage>(256);
    tokio::spawn(async move {
        // TODO: use global ID
        let mut id = 0;
        loop {
            let stats = SystemStats::load(&mut s.info.lock().unwrap());
            let msg = SseMessage::new_json(stats)
                .expect("Could not serialize SystemStats")
                .with_event("system")
                .with_id(id)
                .with_retry(Duration::from_secs(5));

            let Ok(_) = sender.send(msg).await else {
                break;
            };

            id += 1;

            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    });

    (
        StatusCode::OK,
        HeaderMap::from_iter([(CONTENT_TYPE, HeaderValue::from_static("text/event-stream"))]),
        Body::from_channel(receiver),
    )
}

async fn handle_web(_state: Arc<State>, req: Request<Incoming>) -> impl IntoResponse {
    let mut path_builder = req.uri().path().to_owned();

    if path_builder.contains("..") {
        return StatusCode::NOT_FOUND.into_response();
    }

    if path_builder.is_empty() {
        path_builder.push('/');
    }

    if path_builder.ends_with('/') {
        path_builder.push_str("index.html");
    }

    let path = match path_builder.starts_with('/') {
        true => &path_builder[1..],
        false => &path_builder,
    };

    debug!("Handling Web request with final path: {path}");

    let Some(file) = WEB_DIR.get_file(path) else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let content_type = match file
        .path()
        .extension()
        .unwrap_or_default()
        .to_str()
        .unwrap_or_default()
    {
        "html" => Some("text/html"),
        "js" => Some("text/javascript"),
        _ => None,
    };

    let data = file.contents();
    let mut response = data.into_response();
    if let Some(content_type) = content_type {
        response
            .headers_mut()
            .insert(CONTENT_TYPE, HeaderValue::from_static(content_type));
    }

    response
}
