//! Main application entry point for the price tracker
//!
//! Continuously monitors token balances and prices, calculating:
//! - Token balances for two LP tokens
//! - Price ratios in SOL
//! - Price conversion to USD using Pyth Network data

use std::str::FromStr;

use anyhow::{Context, Result};
use futures::join;
use solana_sdk::pubkey::Pubkey;

use price_tracker::{
    config::constants::{LP_TOKEN_A_ADDRESS, LP_TOKEN_B_ADDRESS},
    dbb::dbb::DbHandler,
    price::pyth::fetch_sol_usd_price,
    solana::{client::create_solana_rpc_client, token::get_token_account_balance},
};

#[tokio::main]
async fn main() -> Result<()> {
    let token_str = "woLfjy1RzfvjLNC1AEEqafx1Q3bPDUU3QW6kmBYQjP4";
    let token = Pubkey::from_str(token_str).context("Failed to parse LP1 address")?;
    println!("Token: {}", token);
    let lp1_pubkey = Pubkey::from_str(LP_TOKEN_A_ADDRESS).context("Failed to parse LP1 address")?;
    let lp2_pubkey = Pubkey::from_str(LP_TOKEN_B_ADDRESS).context("Failed to parse LP2 address")?;

    let client = create_solana_rpc_client()?;
    let db = DbHandler::new()?;
    db.create_tables()?;

    loop {
        // Concurrent fetching of balances and price
        let (lp1_balance, lp2_balance, sol_price) = join!(
            get_token_account_balance(&client, lp1_pubkey, 6), // Future Task: Get Decimals from TokenInfo
            get_token_account_balance(&client, lp2_pubkey, 9), // Future Task: Get Decimals from TokenInfo
            fetch_sol_usd_price()
        );

        let lp1_balance = lp1_balance?;
        let lp2_balance = lp2_balance?;
        let sol_price = sol_price?;

        let price_in_sol = lp2_balance / lp1_balance;
        let price_in_usd = price_in_sol * sol_price;

        db.insert_price(token_str, price_in_usd as f32)?;
        let price = db.get_prices(token_str)?;

        if let Some(current_price) = price.last() {
            if *current_price > 0.01 {
                println!("SELL");
            } else {
                println!("WAIT");
            }
        } else {
            println!("Price data is unavailable.");
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;
    }
}
