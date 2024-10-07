use crate::jup_perps::types::RequestChange;
use crate::jup_perps::types::Side;
use solana_sdk::pubkey::Pubkey;

pub struct FindPositionPdaArgs {
    pub custody: Pubkey,
    pub collateral_custody: Pubkey,
    pub owner: Pubkey,
    pub side: Side,
}

pub struct FindRequestPdaArgs {
    pub counter: Option<u64>,
    pub position_pubkey: Pubkey,
    pub request_change: RequestChange,
}

#[derive(Debug, Clone)]
pub struct OpenPositionArgs {
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub side: Side,
    pub custody: Pubkey,
    pub collateral_custody: Pubkey,
    pub request_change: RequestChange,
    pub input_mint: Pubkey,
    pub size_usd_delta: u64,
    pub collateral_token_delta: u64,
    pub price_slippage: u64,
    pub jupiter_minimum_out: String,
    pub counter: Option<u64>,
}
