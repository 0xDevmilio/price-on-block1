//! Solana RPC client configuration and initialization
//!
//! Provides a singleton RPC client instance for Solana network interactions.

use std::env;
use std::sync::Arc;

use dotenv::dotenv;
use solana_client::{client_error::ClientError, nonblocking::rpc_client::RpcClient};
use solana_sdk::commitment_config::CommitmentConfig;

/// Creates and returns a new Solana RPC client instance wrapped in an Arc
///
/// # Returns
/// * `Result<Arc<RpcClient>, ClientError>` - Arc-wrapped RPC client or error if creation fails
///
/// # Errors
/// * Returns `ClientError` if client creation fails
/// * Panics if RPC environment variable is not set in .env file
///
pub fn create_solana_rpc_client() -> Result<Arc<RpcClient>, ClientError> {
    // Load environment variables from .env file
    dotenv().ok();

    // Get RPC endpoint from environment variables
    let rpc = env::var("RPC").expect("RPC endpoint must be set in environment variables");

    // Create and return new RPC client with confirmed commitment level
    Ok(Arc::new(RpcClient::new_with_commitment(
        rpc,
        CommitmentConfig::processed(),
    )))
}
