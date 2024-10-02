//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use crate::jup_perps::types::Side;
use crate::jup_perps::types::RequestType;
use borsh::BorshSerialize;
use borsh::BorshDeserialize;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateIncreasePositionRequestParams {
pub size_usd_delta: u64,
pub collateral_token_delta: u64,
pub side: Side,
pub request_type: RequestType,
pub price_slippage: Option<u64>,
pub jupiter_minimum_out: Option<u64>,
pub trigger_price: Option<u64>,
pub trigger_above_threshold: Option<bool>,
pub counter: u64,
}


