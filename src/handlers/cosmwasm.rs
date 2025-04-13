use cw_orch::daemon::DaemonAsync;
use cw_orch::prelude::*;
use rmcp::{model::*, Error as McpError};
use serde_json::json;

use crate::error::CwOrchMcpError;

pub const QUERY_CONTRACT_DESCRIPTION: &str = "Query a CosmWasm contract";
pub const EXECUTE_CONTRACT_DESCRIPTION: &str = "Execute a message on a CosmWasm contract";
pub const INSTANTIATE_CONTRACT_DESCRIPTION: &str = "Instantiate a CosmWasm contract";

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

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct InstantiateContractRequest {
    #[schemars(description = "The code ID of the contract to instantiate")]
    pub code_id: u64,
    #[schemars(description = "The initialization message as a JSON string")]
    pub init_msg: String,
    #[schemars(description = "Optional label for the contract")]
    pub label: Option<String>,
    #[schemars(description = "Optional admin address for the contract")]
    pub admin: Option<String>,
    #[schemars(description = "Optional funds to send with the instantiation")]
    pub funds: Option<Vec<cw_orch::prelude::Coin>>,
}

pub(super) async fn query_contract_impl(
    daemon: &DaemonAsync,
    request: QueryContractRequest,
) -> Result<CallToolResult, McpError> {
    let query_msg: serde_json::Value = serde_json::from_str(&request.query_msg)
        .map_err(|e| CwOrchMcpError::JsonError(e.to_string()))?;

    let response: serde_json::Value = daemon
        .query(&query_msg, &Addr::unchecked(request.contract_address))
        .await
        .map_err(CwOrchMcpError::DaemonError)?;

    Ok(CallToolResult::success(vec![Content::json(response)?]))
}

pub(super) async fn execute_contract_impl(
    daemon: &DaemonAsync,
    request: ExecuteContractRequest,
) -> Result<CallToolResult, McpError> {
    let execute_msg: serde_json::Value = serde_json::from_str(&request.execute_msg)
        .map_err(|e| CwOrchMcpError::JsonError(e.to_string()))?;

    let response = daemon
        .execute(
            &execute_msg,
            &request.funds.unwrap_or_default(),
            &Addr::unchecked(request.contract_address),
        )
        .await
        .map_err(CwOrchMcpError::DaemonError)?;

    let tx_hash_json = json!({
        "height": response.height,
        "tx_hash": response.txhash,
        "codespace": response.codespace,
        "code": response.code,
        "timestamp": response.timestamp,
    });

    Ok(CallToolResult::success(vec![Content::json(tx_hash_json)?]))
}

pub(super) async fn instantiate_contract_impl(
    daemon: &DaemonAsync,
    request: InstantiateContractRequest,
) -> Result<CallToolResult, McpError> {
    let init_msg: serde_json::Value = serde_json::from_str(&request.init_msg)
        .map_err(|e| CwOrchMcpError::JsonError(e.to_string()))?;

    let admin = request.admin.map(Addr::unchecked);

    let response = daemon
        .instantiate(
            request.code_id,
            &init_msg,
            request.label.as_deref(),
            admin.as_ref(),
            &request.funds.unwrap_or_default(),
        )
        .await
        .map_err(CwOrchMcpError::DaemonError)?;

    let contract_address = response
        .instantiated_contract_address()
        .map_err(|e| CwOrchMcpError::BlockchainError(e.to_string()))?;

    let result_json = json!({
        "height": response.height,
        "tx_hash": response.txhash,
        "codespace": response.codespace,
        "code": response.code,
        "timestamp": response.timestamp,
        "contract_address": contract_address,
    });

    Ok(CallToolResult::success(vec![Content::json(result_json)?]))
}
