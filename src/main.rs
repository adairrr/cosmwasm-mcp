mod config;
mod error;
mod handlers;

use crate::config::McpConfig;
use crate::error::CwOrchMcpError;
use crate::handlers::CwOrchHandler;
use anyhow::Result;
use dotenv::dotenv;
use rmcp::{transport::stdio, ServiceExt};
use tracing::{error, info};
use tracing_subscriber::{self, EnvFilter};

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    // Initialize the tracing subscriber with file and stdout logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env().add_directive(tracing::Level::DEBUG.into()))
        .with_writer(std::io::stderr)
        .with_ansi(false)
        .init();

    tracing::info!("Starting MCP server");

    // Load configuration
    let config = McpConfig::default();

    // Create the MCP handler with DaemonAsync
    let handler = CwOrchHandler::new(&config.server.chain_id, config.server.mnemonic).await?;

    // Serve using stdio transport
    let service = handler.serve(stdio()).await.inspect_err(|e| {
        tracing::error!("serving error: {:?}", e);
    })?;

    service.waiting().await?;
    Ok(())
}
