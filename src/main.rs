mod config;
mod server;
mod session;
mod session_store;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let config = config::ServerConfig::load()?;
    server::run(config).await
}
