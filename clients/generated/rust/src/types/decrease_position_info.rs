//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use borsh::BorshSerialize;
use borsh::BorshDeserialize;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DecreasePositionInfo {
pub price: u64,
pub liquidation_price: u64,
pub fee_usd: u64,
pub collateral_usd: u64,
pub has_profit: bool,
pub pnl_delta: u64,
pub transfer_amount_usd: u64,
pub transfer_token: u64,
}


