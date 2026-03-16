use anyhow::Context as _;
use wassel_plugin_stack::Stack;

use crate::server::Server;

pub(crate) mod router;
pub(crate) mod server;
pub(crate) mod service;
pub(crate) mod sse;
pub(crate) mod stats;

pub async fn run_admin_dashboard(stack: Stack) -> anyhow::Result<()> {
    let server = Server::new(stack);
    server.serve().await.context("Serving")?;

    Ok(())
}
