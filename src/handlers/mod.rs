use crate::error::CwOrchMcpError;
use cw_orch::daemon::networks::parse_network;
use cw_orch::daemon::queriers::Bank;
use cw_orch::daemon::{DaemonAsync, DaemonAsyncBuilder};
use cw_orch::prelude::*;
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
    #[schemars(description = "The denomination of the balance to query (optional)")]
    pub denom: Option<String>,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct QueryContractRequest {
    #[schemars(description = "The contract address to query")]
    pub contract_address: String,
    #[schemars(description = "The query message as a JSON string")]
    pub query_msg: String,
}

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct ExecuteContractRequest {
    #[schemars(description = "The contract address to execute on")]
    pub contract_address: String,
    #[schemars(description = "The execute message as a JSON string")]
    pub execute_msg: String,
    #[schemars(description = "Optional funds to send with the execution")]
    pub funds: Option<Vec<cw_orch::prelude::Coin>>,
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
        let bank = Bank::new_async(self.daemon.channel());

        let balance = bank
            ._balance(&Addr::unchecked(request.address), request.denom)
            .await
            .map_err(|e| CwOrchMcpError::DaemonError(e))?;

        Ok(CallToolResult::success(vec![Content::json(balance)?]))
    }

    #[tool(description = "Get the current block height")]
    async fn get_block_height(&self) -> Result<CallToolResult, McpError> {
        // Use the daemon's methods to get the block height
        let height = self
            .daemon
            .block_info()
            .await
            .map_err(|e| CwOrchMcpError::DaemonError(e))?
            .height;

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

    #[tool(description = "Query a CosmWasm contract")]
    async fn query_contract(
        &self,
        #[tool(aggr)] request: QueryContractRequest,
    ) -> Result<CallToolResult, McpError> {
        // Parse the query message from JSON
        let query_msg: serde_json::Value = serde_json::from_str(&request.query_msg)
            .map_err(|e| CwOrchMcpError::JsonError(e.to_string()))?;

        // Query the contract
        let response: serde_json::Value = self
            .daemon
            .query(&query_msg, &Addr::unchecked(request.contract_address))
            .await
            .map_err(|e| CwOrchMcpError::DaemonError(e))?;

        Ok(CallToolResult::success(vec![Content::json(response)?]))
    }

    #[tool(description = "Execute a message on a CosmWasm contract")]
    async fn execute_contract(
        &self,
        #[tool(aggr)] request: ExecuteContractRequest,
    ) -> Result<CallToolResult, McpError> {
        // Parse the execute message from JSON
        let execute_msg: serde_json::Value = serde_json::from_str(&request.execute_msg)
            .map_err(|e| CwOrchMcpError::JsonError(e.to_string()))?;

        // Execute the contract
        let response = self
            .daemon
            .execute(
                &execute_msg,
                &request.funds.unwrap_or_default(),
                &Addr::unchecked(request.contract_address),
            )
            .await
            .map_err(|e| CwOrchMcpError::DaemonError(e))?;

        let tx_hash_json = serde_json::json!({
           "height": response.height,
           "tx_hash": response.txhash,
           "codespace": response.codespace,
           "code": response.code,
           "timestamp": response.timestamp,
        });
        Ok(CallToolResult::success(vec![Content::json(tx_hash_json)?]))
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
