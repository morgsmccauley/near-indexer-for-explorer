use anyhow::Context;

use near_jsonrpc_client::JsonRpcClient;

pub(crate) async fn get_account_balance(
    rpc_client: &JsonRpcClient,
    account_id: &near_primitives::types::AccountId,
    block_height: &near_primitives::types::BlockHeight,
) -> anyhow::Result<near_primitives::types::Balance> {
    get_account_view_for_block_height(rpc_client, account_id, block_height)
        .await
        .map(|account| account.amount)
        .with_context(|| format!("Unable to get account balance for {}", account_id))
}

pub(crate) async fn get_contract_code_hash(
    rpc_client: &JsonRpcClient,
    account_id: &near_primitives::types::AccountId,
    block_height: &near_primitives::types::BlockHeight,
) -> anyhow::Result<near_primitives::hash::CryptoHash> {
    get_account_view_for_block_height(rpc_client, account_id, block_height)
        .await
        .map(|account| account.code_hash)
        .with_context(|| format!("Unable to get contract code hash for {}", account_id))
}

async fn get_account_view_for_block_height(
    rpc_client: &JsonRpcClient,
    account_id: &near_primitives::types::AccountId,
    block_height: &near_primitives::types::BlockHeight,
) -> anyhow::Result<near_primitives::views::AccountView> {
    let block_reference = near_primitives::types::BlockReference::BlockId(
        near_primitives::types::BlockId::Height(*block_height),
    );
    let request = near_primitives::views::QueryRequest::ViewAccount {
        account_id: account_id.clone(),
    };
    let query = near_jsonrpc_client::methods::query::RpcQueryRequest {
        block_reference,
        request,
    };

    let account_response = rpc_client.call(query).await.with_context(|| {
        format!(
            "Failed to deliver ViewAccount for account {}, block {}",
            account_id, block_height
        )
    })?;

    match account_response.kind {
        near_jsonrpc_primitives::types::query::QueryResponseKind::ViewAccount(account) => {
            Ok(account)
        }
        _ => anyhow::bail!(
            "Failed to extract ViewAccount response for account {}, block {}",
            account_id,
            block_height
        ),
    }
}
