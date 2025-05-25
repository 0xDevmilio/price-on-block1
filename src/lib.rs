//! Price tracking application for Solana tokens
//!
//! This crate provides functionality to:
//! - Track token prices on Solana
//! - Fetch SOL/USD prices from Pyth Network
//! - Calculate token prices in both SOL and USD

pub mod config;
pub mod dbb {
    pub mod dbb;
}
pub mod price;
pub mod solana;
