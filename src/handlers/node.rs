use cw_orch::daemon::{queriers::Node, DaemonAsync};
use rmcp::{model::*, Error as McpError};
use serde_json::json;

use crate::error::CwOrchMcpError;

pub const GET_BLOCK_INFO_DESCRIPTION: &str =
    "Get the current block information (height, time, chain id)";
pub const GET_CHAIN_ID_DESCRIPTION: &str = "Get the chain ID";
pub const GET_TX_DESCRIPTION: &str = "Get transaction information by hash";

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct GetTxRequest {
    #[schemars(description = "The transaction hash to query")]
    pub tx_hash: String,
}

pub(super) async fn get_block_info_impl(daemon: &DaemonAsync) -> Result<CallToolResult, McpError> {
    let block = daemon
        .block_info()
        .await
        .map_err(CwOrchMcpError::DaemonError)?;

    Ok(CallToolResult::success(vec![Content::json(block)?]))
}

pub(super) fn get_chain_id_impl(daemon: &DaemonAsync) -> Result<CallToolResult, McpError> {
    let chain_id = daemon.chain_info().chain_id.clone();
    Ok(CallToolResult::success(vec![Content::text(chain_id)]))
}

pub(super) async fn get_tx_impl(
    daemon: &DaemonAsync,
    request: GetTxRequest,
) -> Result<CallToolResult, McpError> {
    let node = Node::new_async(daemon.channel());

    let tx = node
        ._find_tx(request.tx_hash.clone())
        .await
        .map_err(CwOrchMcpError::DaemonError)?;

    let tx_hash_json = json!({
        "height": tx.height,
        "tx_hash": tx.txhash,
        "codespace": tx.codespace,
        "code": tx.code,
        "timestamp": tx.timestamp,
    });

    Ok(CallToolResult::success(vec![Content::json(tx_hash_json)?]))
}
