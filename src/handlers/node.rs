use cw_orch::daemon::DaemonAsync;
use rmcp::{model::*, Error as McpError};

use crate::error::CwOrchMcpError;

pub const GET_BLOCK_HEIGHT_DESCRIPTION: &str = "Get the current block height";
pub const GET_CHAIN_ID_DESCRIPTION: &str = "Get the chain ID";

pub(super) async fn get_block_height_impl(
    daemon: &DaemonAsync,
) -> Result<CallToolResult, McpError> {
    let height = daemon
        .block_info()
        .await
        .map_err(CwOrchMcpError::DaemonError)?
        .height;

    Ok(CallToolResult::success(vec![Content::text(
        height.to_string(),
    )]))
}

pub(super) fn get_chain_id_impl(daemon: &DaemonAsync) -> Result<CallToolResult, McpError> {
    let chain_id = daemon.chain_info().chain_id.clone();
    Ok(CallToolResult::success(vec![Content::text(chain_id)]))
}
