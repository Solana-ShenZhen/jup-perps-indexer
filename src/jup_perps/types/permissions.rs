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
pub struct Permissions {
pub allow_swap: bool,
pub allow_add_liquidity: bool,
pub allow_remove_liquidity: bool,
pub allow_increase_position: bool,
pub allow_decrease_position: bool,
pub allow_collateral_withdrawal: bool,
pub allow_liquidate_position: bool,
}


