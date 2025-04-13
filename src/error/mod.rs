use rmcp::{model::ErrorCode, Error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CwOrchMcpError {
    #[error("MCP error: {0}")]
    McpError(#[from] rmcp::Error),

    #[error("CosmWasm error: {0}")]
    CosmWasmError(String),

    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Blockchain error: {0}")]
    BlockchainError(String),

    #[error("Internal error: {0}")]
    InternalError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}

impl From<CwOrchMcpError> for Error {
    fn from(err: CwOrchMcpError) -> Self {
        Error {
            code: ErrorCode(-32000),
            message: std::borrow::Cow::Owned(err.to_string()),
            data: None,
        }
    }
}
