//! Pyth Network price feed integration
//!
//! Fetches and processes real-time price data from Pyth Network,
//! specifically for SOL/USD price feeds.

use std::sync::OnceLock;

use anyhow::{anyhow, Context, Result};
use reqwest::Client;
use serde_json::Value;

static HTTP_CLIENT: OnceLock<Client> = OnceLock::new();

/// Fetches the current SOL/USD price from Pyth Network
///
/// # Returns
/// * `Result<f64>` - The current SOL/USD price
///
/// # Errors
/// * Network request failures
/// * JSON parsing errors
/// * Missing or invalid price data
pub async fn fetch_sol_usd_price() -> Result<f64> {
    // SOL/USD price feed ID on Pyth Network
    const SOL_USD_PRICE_FEED: &str =
        "ef0d8b6fda2ceba41da15d4095d1da392a0d2f8ed0c6c7bc0f4cfac8c280b56d";
    const PYTH_API_BASE: &str = "https://hermes.pyth.network/v2/updates/price/latest";

    let client = HTTP_CLIENT.get_or_init(Client::new);
    let url = format!("{}?ids%5B%5D={}", PYTH_API_BASE, SOL_USD_PRICE_FEED);

    let data: Value = client
        .get(&url)
        .send()
        .await
        .context("Failed to fetch data from Pyth Network")?
        .json()
        .await
        .context("Failed to parse JSON response")?;

    // Extract price data with detailed error handling
    let price_float = data["parsed"]
        .as_array()
        .ok_or_else(|| anyhow!("Missing 'parsed' array in response"))?
        .first()
        .ok_or_else(|| anyhow!("Empty price data array"))?;

    // Extract price and exponent
    let price_str = price_float["price"]["price"]
        .as_str()
        .ok_or_else(|| anyhow!("Invalid price format"))?;

    let expo = price_float["price"]["expo"]
        .as_i64()
        .ok_or_else(|| anyhow!("Invalid exponent format"))?;

    // Calculate final price
    let price = price_str
        .parse::<f64>()
        .context("Failed to parse price string")?
        * 10f64.powi(expo as i32);

    log::debug!("SOL/USD Price: ${:.2}", price);

    Ok(price)
}
