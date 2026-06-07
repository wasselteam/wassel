use anyhow::Context as _;
use hyper_util::{
    rt::{TokioExecutor, TokioIo},
    server::conn::auto,
};
use tokio::net::{TcpListener, TcpStream};
use tracing::{error, info};

use wassel_plugin_stack::Stack;

use crate::config::Config;

pub struct Server {
    config: Config,
    stack: Stack,
}

impl Server {
    pub fn new(config: Config, stack: Stack) -> Self {
        Self { config, stack }
    }

    pub async fn serve(&self) -> anyhow::Result<()> {
        let listener = self.bind().await?;

        loop {
            let (tcp, _) = listener.accept().await.context("Accepting connection")?;
            let io = TokioIo::new(tcp);
            let stack = self.stack.clone();
            tokio::task::spawn(Self::handle_connection(io, stack));
        }
    }

    async fn bind(&self) -> anyhow::Result<TcpListener> {
        let addr = format!(
            "{host}:{port}",
            host = &self.config.host,
            port = self.config.port
        );
        info!("Starting server at {addr}");
        TcpListener::bind(&addr).await.context("Binding to {addr}")
    }

    async fn handle_connection(io: TokioIo<TcpStream>, stack: Stack) {
        if let Err(e) = auto::Builder::new(TokioExecutor::new())
            .serve_connection_with_upgrades(io, stack)
            .await
        {
            error!("Error serving: {e:?}");
        }
    }
}
