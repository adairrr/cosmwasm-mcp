use crate::error::CwOrchMcpError;
use cw_orch::daemon::networks::parse_network;
use cw_orch::daemon::{DaemonAsync, DaemonAsyncBuilder};
use rmcp::{
    const_string, model::*, schemars, service::RequestContext, tool, Error as McpError, RoleServer,
    ServerHandler,
};
use serde_json::json;
use std::sync::Arc;

#[derive(Clone)]
pub struct CwOrchHandler {
    daemon: Arc<DaemonAsync>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct QueryBalanceRequest {
    #[schemars(description = "The address to query the balance for")]
    pub address: String,
}

impl CwOrchHandler {
    pub async fn new(chain_id: &str, mnemonic: Option<String>) -> Result<Self, McpError> {
        // Create a proper ChainInfoOwned object
        let chain_info =
            parse_network(chain_id).map_err(|e| CwOrchMcpError::BlockchainError(e.to_string()))?;

        let mut builder = DaemonAsyncBuilder::new(chain_info);

        if let Some(mnemonic) = mnemonic {
            builder.mnemonic(mnemonic);
        }

        let daemon = builder
            .build()
            .await
            .map_err(|e| CwOrchMcpError::BlockchainError(e.to_string()))?;

        Ok(Self {
            daemon: Arc::new(daemon),
        })
    }
}

#[tool(tool_box)]
impl CwOrchHandler {
    #[tool(description = "Query the balance of an address")]
    async fn query_balance(
        &self,
        #[tool(aggr)] request: QueryBalanceRequest,
    ) -> Result<CallToolResult, McpError> {
        // Use the daemon's methods to query the balance
        // Note: This is a placeholder - you'll need to implement the actual balance query
        // based on the DaemonAsync API
        let balance = "1000000uosmo".to_string(); // Placeholder

        Ok(CallToolResult::success(vec![Content::text(balance)]))
    }

    #[tool(description = "Get the current block height")]
    async fn get_block_height(&self) -> Result<CallToolResult, McpError> {
        // Use the daemon's methods to get the block height
        // Note: This is a placeholder - you'll need to implement the actual block height query
        // based on the DaemonAsync API
        let height = 1000000; // Placeholder

        Ok(CallToolResult::success(vec![Content::text(
            height.to_string(),
        )]))
    }

    #[tool(description = "Get the chain ID")]
    fn get_chain_id(&self) -> Result<CallToolResult, McpError> {
        // Get the chain ID from the daemon
        // Note: This is a placeholder - you'll need to implement the actual chain ID query
        // based on the DaemonAsync API
        let chain_id = self.daemon.chain_info().chain_id.clone();

        Ok(CallToolResult::success(vec![Content::text(chain_id)]))
    }
}

#[tool(tool_box)]
impl ServerHandler for CwOrchHandler {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            protocol_version: ProtocolVersion::V_2024_11_05,
            capabilities: ServerCapabilities::builder()
                .enable_prompts()
                .enable_resources()
                .enable_tools()
                .build(),
            server_info: Implementation::from_build_env(),
            instructions: Some("A CosmWasm blockchain interaction server".into()),
        }
    }
}
