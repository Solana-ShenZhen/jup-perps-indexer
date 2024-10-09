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
    pub funding_account: Pubkey,
    pub perpetuals: Pubkey,
    pub pool: Pubkey,
    pub position: Pubkey,
    pub position_request: Pubkey,
    pub position_request_ata: Pubkey,
    pub custody: Pubkey,
    pub collateral_custody: Pubkey,
    pub input_mint: Pubkey,
    pub referral: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub size_usd_delta: u64,
    pub collateral_token_delta: u64,
    pub price_slippage: u64,
    pub jupiter_minimum_out: Option<u64>,
    pub counter: u64,
    pub program: Pubkey,
    pub side: Side,
    pub paying: u64,
}

#[derive(Debug, Clone)]
pub struct InputArgs {
    pub owner: Pubkey,
    pub input_mint: Pubkey,
    pub leverage: u16,
    pub paying: u64,
    pub collateral_token_delta: u64,
    pub price_slippage: u64,
    pub jupiter_minimum_out: Option<u64>,
    pub request_change: RequestChange,
    pub side: Side,
}
