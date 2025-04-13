use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    /// The chain ID to connect to
    pub chain_id: String,
    /// Mnemonic for the wallet (optional, can use env var instead)
    pub mnemonic: Option<String>,
    /// Whether to write state on every change
    pub write_on_change: Option<bool>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            chain_id: "pion-1".to_string(),
            mnemonic: None,
            write_on_change: Some(true),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpConfig {
    /// The port to listen on for MCP connections
    pub port: u16,
    /// The host to bind to
    pub host: String,
    /// Server configuration
    pub server: ServerConfig,
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            port: 8080,
            host: "127.0.0.1".to_string(),
            server: ServerConfig::default(),
        }
    }
}
