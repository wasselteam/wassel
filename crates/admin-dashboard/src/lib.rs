use anyhow::Context as _;
use tokio::sync::broadcast;
use wassel_plugin_stack::Stack;
use wassel_subscriber::LogMessage;

use crate::server::Server;

pub(crate) mod router;
pub(crate) mod server;
pub(crate) mod service;
pub(crate) mod sse;
pub(crate) mod stats;

pub async fn run_admin_dashboard(
    stack: Stack,
    log_receiver: broadcast::Receiver<LogMessage>,
) -> anyhow::Result<()> {
    let server = Server::new(stack, log_receiver);
    server.serve().await.context("Serving")?;

    Ok(())
}
