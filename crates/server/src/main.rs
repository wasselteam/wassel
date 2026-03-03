use tracing::level_filters::LevelFilter;
use tracing_subscriber::EnvFilter;
use wassel_plugin_stack::Stack;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let filter = EnvFilter::builder()
        .with_default_directive(LevelFilter::INFO.into())
        .from_env_lossy()
        .add_directive("wasmtime=info".parse()?)
        .add_directive("cranelift_codegen=info".parse()?)
        .add_directive("cranelift_frontend=info".parse()?);

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .init();

    let stack = Stack::load(".").await?;

    wassel_server::run_server(stack).await
}
