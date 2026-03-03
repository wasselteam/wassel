use anyhow::Context as _;
use hyper::server::conn::http1;
use hyper_util::rt::{TokioIo, TokioTimer};
use tokio::net::TcpListener;
use tracing::{error, info};
use wassel_plugin_stack::Stack;

use crate::service::AdminService;

pub struct Server {
    service: AdminService,
}

impl Server {
    pub fn new(stack: Stack) -> Self {
        Self {
            service: AdminService::new(stack),
        }
    }

    pub async fn serve(&self) -> anyhow::Result<()> {
        let addr = "127.0.0.1:3511";
        info!("Starting server at {addr}");
        let listener = TcpListener::bind(&addr)
            .await
            .context("Binding to {addr}")?;

        loop {
            let (tcp, _) = listener.accept().await.context("Accepting connection")?;
            let io = TokioIo::new(tcp);
            let s = self.service.clone();

            tokio::task::spawn(async move {
                if let Err(e) = http1::Builder::new()
                    .timer(TokioTimer::new())
                    .serve_connection(io, s)
                    .await
                {
                    error!("Error serving: {e:?}");
                }
            });
        }
    }
}
