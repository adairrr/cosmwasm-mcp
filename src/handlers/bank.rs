use cw_orch::daemon::queriers::Bank;
use cw_orch::prelude::*;
use rmcp::{model::*, Error as McpError};

use crate::error::CwOrchMcpError;

pub const QUERY_BALANCE_DESCRIPTION: &str = "Query the balance of an address";

#[derive(Debug, serde::Deserialize, schemars::JsonSchema)]
pub struct QueryBalanceRequest {
    #[schemars(description = "The address to query the balance for")]
    pub address: String,
    #[schemars(description = "The denomination of the balance to query (optional)")]
    pub denom: Option<String>,
}

pub(super) async fn query_balance_impl(
    daemon: &DaemonAsync,
    request: QueryBalanceRequest,
) -> Result<CallToolResult, McpError> {
    let bank = Bank::new_async(daemon.channel());

    let balance = bank
        ._balance(&Addr::unchecked(request.address), request.denom)
        .await
        .map_err(CwOrchMcpError::DaemonError)?;

    Ok(CallToolResult::success(vec![Content::json(balance)?]))
}
