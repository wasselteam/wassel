use anyhow::Context as _;
use wassel_plugin_stack::Stack;

use crate::server::Server;

mod server;
mod service;
mod stats;

pub async fn run_admin_dashboard(stack: Stack) -> anyhow::Result<()> {
    let server = Server::new(stack);
    server.serve().await.context("Serving")?;

    Ok(())
}
