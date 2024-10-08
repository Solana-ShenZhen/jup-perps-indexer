//! This code was AUTOGENERATED using the kinobi library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun kinobi to update it.
//!
//! <https://github.com/kinobi-so/kinobi>
//!

use crate::jup_perps::types::RequestType;
use crate::jup_perps::types::Side;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateIncreasePositionRequestParams {
    /// The change in position size in USD
    pub size_usd_delta: u64,
    /// The change in collateral amount
    pub collateral_token_delta: u64,
    /// The side of the position (Long or Short)
    pub side: Side,
    /// The type of request (Market or Trigger)
    pub request_type: RequestType,
    /// Maximum allowed price slippage
    pub price_slippage: Option<u64>,
    /// Minimum output amount for Jupiter swap
    pub jupiter_minimum_out: Option<u64>,
    /// Price at which to trigger the request (for Trigger requests)
    pub trigger_price: Option<u64>,
    /// Whether to trigger above or below the threshold (for Trigger requests)
    pub trigger_above_threshold: Option<bool>,
    /// Counter for tracking request iterations
    pub counter: u64,
}
