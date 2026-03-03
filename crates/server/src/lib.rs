use anyhow::Context;
use config::Config;
use server::Server;
use wassel_plugin_stack::Stack;

mod config;
mod server;

pub async fn run_server(stack: Stack) -> anyhow::Result<()> {
    let config = Config::load().context("Loading config")?;
    let server = Server::new(config, stack);
    server.serve().await.context("Serving")?;

    Ok(())
}
