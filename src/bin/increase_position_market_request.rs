use dotenv::dotenv;
use jup_perps_indexer::jup_perps::accounts::custody;
use jup_perps_indexer::jup_perps::accounts::perpetuals;
use jup_perps_indexer::jup_perps::accounts::pool;
use jup_perps_indexer::jup_perps::accounts::program_accounts;
use jup_perps_indexer::jup_perps::accounts::{Position, PositionRequest};
use jup_perps_indexer::jup_perps::types::RequestChange;
use jup_perps_indexer::jup_perps::types::Side;
use jup_perps_indexer::rust_client::instructions::create_increase_position_market_request::open_position;
use jup_perps_indexer::rust_client::transaction_utils::get_ata;
use jup_perps_indexer::rust_client::types::{InputArgs, OpenPositionArgs};
use solana_program::pubkey;
use solana_sdk::pubkey::Pubkey;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let args = InputArgs {
        owner: pubkey!("HPowdMHegSM3fNFMe8pyMRPDAgYGcyz3VL7zGwwnUcX7"),
        input_mint: pubkey!("So11111111111111111111111111111111111111112"),
        leverage: 10,
        paying: 1000,
        collateral_token_delta: 0,
        price_slippage: 0,
        jupiter_minimum_out: None,
        request_change: RequestChange::Increase,
        side: Side::Long,
    };


    let funding_account = get_ata(&args.owner, &args.input_mint);

    let perpetuals = perpetuals::Perpetuals::ADDRESS;
    let pool = pool::Pool::ADDRESS;

    let custody = match args.input_mint.to_string().as_str() {
        "So11111111111111111111111111111111111111112" => custody::Custody::SOL_ADDRESS,
        "7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs" => custody::Custody::ETH_ADDRESS,
        "3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh" => custody::Custody::BTC_ADDRESS,
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v" => custody::Custody::USDC_ADDRESS,
        "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB" => custody::Custody::USDT_ADDRESS,
        _ => return Err("Unsupported input mint".into()),
    };

    let collateral_custody = custody.clone();
    let position_pda = get_position_pda(custody, collateral_custody, args.owner, args.side);

    let position_request_pda = get_position_request_pda(position_pda.0, args.request_change);
    let size_usd_delta = (args.paying * args.leverage as u64 / 10) as u64;
    let position_request_ata = get_ata(&position_request_pda.0, &args.input_mint);
    let input_mint = args.input_mint;
    let referral = program_accounts::REFERRER_ADDRESS;
    let token_program = program_accounts::TOKEN_PROGRAM_ADDRESS;
    let associated_token_program = program_accounts::ASSOCIATED_TOKEN_PROGRAM_ADDRESS;
    let system_program = program_accounts::SYSTEM_PROGRAM_ADDRESS;
    let event_authority = program_accounts::EVENT_AUTHORITY_ADDRESS;
    let program = program_accounts::PROGRAM_ADDRESS;

    open_position(&OpenPositionArgs {
        owner: args.owner,
        funding_account,
        perpetuals,
        pool,
        position: position_pda.0,
        position_request: position_request_pda.0,
        position_request_ata,
        custody,
        collateral_custody,
        input_mint,
        referral,
        token_program,
        associated_token_program,
        system_program,
        event_authority,
        size_usd_delta: size_usd_delta,
        collateral_token_delta: args.collateral_token_delta,
        price_slippage: args.price_slippage,
        jupiter_minimum_out: args.jupiter_minimum_out.clone(),
        counter: position_request_pda.2,
        program,
        side: args.side,
        paying: args.paying,
    })?;

    Ok(())
}

fn get_position_request_pda(position: Pubkey, request_change: RequestChange) -> (Pubkey, u8, u64) {
    let (_, bump, counter) = PositionRequest::find_pda(None, position, request_change);

    let pda = PositionRequest::create_pda(Some(counter), position, request_change, bump)
        .expect("create failed");

    (pda, bump, counter)
}

fn get_position_pda(
    custody: Pubkey,
    collateral_custody: Pubkey,
    wallet_address: Pubkey,
    side: Side,
) -> (Pubkey, u8) {
    let (pda, bump) = Position::find_pda(custody, collateral_custody, wallet_address, side);

    let created_pda = Position::create_pda(custody, collateral_custody, wallet_address, side, bump)
        .expect("create_failed");

    (pda, bump)
}
