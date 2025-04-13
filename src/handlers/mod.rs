mod bank;
mod cosmwasm;
mod node;

use cw_orch::daemon::networks::parse_network;
use cw_orch::daemon::DaemonAsync;
use rmcp::{model::*, tool, Error as McpError, ServerHandler};
use std::sync::Arc;

use crate::error::CwOrchMcpError;
pub use bank::QueryBalanceRequest;
pub use cosmwasm::{ExecuteContractRequest, InstantiateContractRequest, QueryContractRequest};

#[derive(Clone)]
pub struct CwOrchHandler {
    daemon: Arc<DaemonAsync>,
}

impl CwOrchHandler {
    pub async fn new(chain_id: &str, mnemonic: Option<String>) -> Result<Self, McpError> {
        let chain_info =
            parse_network(chain_id).map_err(|e| CwOrchMcpError::BlockchainError(e.to_string()))?;

        let mut builder = cw_orch::daemon::DaemonAsyncBuilder::new(chain_info);

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
    /////////////////////////
    ////  Bank Handler   ////
    ////////////////////////
    #[tool(description = bank::QUERY_BALANCE_DESCRIPTION)]
    async fn query_balance(
        &self,
        #[tool(aggr)] request: QueryBalanceRequest,
    ) -> Result<CallToolResult, McpError> {
        bank::query_balance_impl(&self.daemon, request).await
    }

    /////////////////////////
    ////  Node Handler   /////
    /// //////////////////////
    #[tool(description = node::GET_BLOCK_INFO_DESCRIPTION)]
    async fn get_block_info(&self) -> Result<CallToolResult, McpError> {
        node::get_block_info_impl(&self.daemon).await
    }

    #[tool(description = node::GET_CHAIN_ID_DESCRIPTION)]
    fn get_chain_id(&self) -> Result<CallToolResult, McpError> {
        node::get_chain_id_impl(&self.daemon)
    }

    #[tool(description = node::GET_TX_DESCRIPTION)]
    async fn get_tx(
        &self,
        #[tool(aggr)] request: node::GetTxRequest,
    ) -> Result<CallToolResult, McpError> {
        node::get_tx_impl(&self.daemon, request).await
    }

    /////////////////////////
    ////  CosmWasm Handler ///
    /////////////////////////
    #[tool(description = cosmwasm::QUERY_CONTRACT_DESCRIPTION)]
    async fn query_contract(
        &self,
        #[tool(aggr)] request: QueryContractRequest,
    ) -> Result<CallToolResult, McpError> {
        cosmwasm::query_contract_impl(&self.daemon, request).await
    }

    #[tool(description = cosmwasm::EXECUTE_CONTRACT_DESCRIPTION)]
    async fn execute_contract(
        &self,
        #[tool(aggr)] request: ExecuteContractRequest,
    ) -> Result<CallToolResult, McpError> {
        cosmwasm::execute_contract_impl(&self.daemon, request).await
    }

    #[tool(description = cosmwasm::INSTANTIATE_CONTRACT_DESCRIPTION)]
    async fn instantiate_contract(
        &self,
        #[tool(aggr)] request: InstantiateContractRequest,
    ) -> Result<CallToolResult, McpError> {
        cosmwasm::instantiate_contract_impl(&self.daemon, request).await
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
