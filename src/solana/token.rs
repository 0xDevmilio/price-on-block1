//! Token account balance utilities for Solana
//!
//! Provides functions to fetch and calculate token balances
//! with proper decimal adjustment.

use std::sync::Arc;

use solana_client::{client_error::ClientError, nonblocking::rpc_client::RpcClient};
use solana_sdk::pubkey::Pubkey;

/// Fetches the token account balance for a given pubkey
///
/// # Arguments
/// * `pubkey` - The public key of the token account
/// * `decimals` - Number of decimal places for the token
///
/// # Returns
/// * `Result<f64>` - The token balance as a floating point number
///
/// # Errors
/// * Returns error if RPC client creation fails
/// * Returns error if balance fetch fails
pub async fn get_token_account_balance(
    client: &Arc<RpcClient>,
    pubkey: Pubkey,
    decimals: u8,
) -> Result<f64, ClientError> {
    let balance = client.get_token_account_balance(&pubkey).await?;
    let raw_balance = balance.amount.parse::<u64>().unwrap_or(0);

    // Convert to decimal by dividing by 10^decimals
    let balance_result = raw_balance as f64 / 10_f64.powi(decimals as i32);

    Ok(balance_result)
}
