mod config;

use anyhow::{Context, Result, bail};
use axum::{Router, routing::get};
use sqlx::postgres::PgConnectOptions;
use tracing_subscriber::EnvFilter;

use config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    let config = Config::from_env()?;
    if !config.database_url.starts_with("postgres://")
        && !config.database_url.starts_with("postgresql://")
    {
        bail!("DATABASE_URL must be a valid postgres:// or postgresql:// connection URL");
    }
    // Validate configuration without opening a database connection yet.
    let _database_options = config
        .database_url
        .parse::<PgConnectOptions>()
        .map_err(|_| anyhow::anyhow!("DATABASE_URL must be a valid PostgreSQL connection URL"))?;

    let log_filter = EnvFilter::try_new(&config.rust_log)
        .context("RUST_LOG must be a valid tracing filter, such as info or info,backend=debug")?;
    tracing_subscriber::fmt().with_env_filter(log_filter).init();

    let app = Router::new()
        .route("/", get(root))
        .route("/health", get(health));

    let listener = tokio::net::TcpListener::bind(("0.0.0.0", config.port))
        .await
        .with_context(|| format!("Could not bind server to port {}", config.port))?;

    tracing::info!(address = %listener.local_addr()?, "Backend listening");

    axum::serve(listener, app).await.context("Server failed")?;
    Ok(())
}

async fn root() -> &'static str {
    "Hello from Rust"
}

async fn health() -> &'static str {
    "OK"
}
