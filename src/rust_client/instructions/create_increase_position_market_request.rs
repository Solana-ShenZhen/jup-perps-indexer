// use self::fee_estimation::CREATE_POOL_COMPUTE_UNIT;
use crate::jup_perps::accounts::Position;
use crate::jup_perps::accounts::PositionRequest;
use crate::jup_perps::accounts::perpetuals;
use crate::jup_perps::accounts::custody;
use crate::jup_perps::accounts::pool;
use crate::jup_perps::accounts::program_accounts;
use crate::jup_perps::types::RequestChange;
use crate::jup_perps::types::Side;
use crate::rust_client::types::OpenPositionArgs;
use crate::rust_client::types::FindPositionPdaArgs;
use crate::rust_client::types::FindRequestPdaArgs;
use anchor_spl::associated_token::get_associated_token_address;
use solana_rpc_client::rpc_client::RpcClient;
use crate::rust_client::transaction_utils::create_ata;
use solana_sdk::pubkey::Pubkey;
use crate::jup_perps::instructions::create_increase_position_market_request;

// entrypoint!(open_position);

pub fn open_position(args: &OpenPositionArgs) -> Result<(), Box<dyn std::error::Error>> {
    let owner = args.owner;

    let position_pda = create_position_account(FindPositionPdaArgs {
        custody: args.custody,
        collateral_custody: args.collateral_custody,
        owner: args.owner,
        side: args.side,
    }).unwrap();

    let position_request_pda = create_position_request_account(FindRequestPdaArgs {
        counter: args.counter,
        position_pubkey: position_pda,
        request_change: args.request_change,
    }, FindPositionPdaArgs {
        custody: args.custody,
        collateral_custody: args.collateral_custody,
        owner: args.owner,
        side: args.side,
    }).unwrap();

    let funding_account = create_ata(&args.owner, &args.input_mint);

    let perpetuals = perpetuals::Perpetuals::ADDRESS;

    let pool = pool::Pool::ADDRESS;

    let position_request_ata = create_ata(&position_request_pda, &args.input_mint);

    let custody = custody::Custody::ADDRESS;

    let collateral_custody = custody::Custody::ADDRESS;

    let input_mint = args.input_mint;

    let referral = program_accounts::REFERRER_ADDRESS;

    let token_program = program_accounts::TOKEN_PROGRAM_ADDRESS;

    let associated_token_program = program_accounts::ASSOCIATED_TOKEN_PROGRAM_ADDRESS;

    let system_program = program_accounts::SYSTEM_PROGRAM_ADDRESS;

    let event_authority = program_accounts::EVENT_AUTHORITY_ADDRESS;
    
    let program = program_accounts::PROGRAM_ADDRESS;

    create_increase_position_market_request::CreateIncreasePositionMarketRequest {
        owner: owner,
        funding_account: funding_account,
        perpetuals: perpetuals,
        pool: pool,
        position: position_pda,
        position_request: position_request_pda,
        position_request_ata: position_request_ata,
        custody: custody,
        collateral_custody: collateral_custody,
        input_mint: input_mint,
        referral: Some(referral),
        token_program: token_program,
        associated_token_program: associated_token_program,
        system_program: system_program,
        event_authority: event_authority,
        program: program,
    };

    Ok(())
}

pub fn create_position_account(args: FindPositionPdaArgs) -> Result<Pubkey, Box<dyn std::error::Error>> {
    let (position_pda, bump) =
        Position::find_pda(args.custody, args.collateral_custody, args.owner, args.side);

    Position::create_pda(args.custody, args.collateral_custody, args.owner, args.side, bump)?;

    Ok(position_pda)
}

pub fn create_position_request_account(
    requestargs: FindRequestPdaArgs, 
    positionargs: FindPositionPdaArgs,
) -> Result<Pubkey, Box<dyn std::error::Error>> {
    let (position_pda, position_bump) =
        Position::find_pda(positionargs.custody, positionargs.collateral_custody, positionargs.owner, positionargs.side);

    let (position_request_pda, position_request_bump) = PositionRequest::find_pda(
        Some(requestargs.counter.unwrap().to_string().parse()?),
        position_pda,
        requestargs.request_change,           
    );

    let position_request_account = PositionRequest::create_pda(
        Some(requestargs.counter.unwrap().to_string().parse()?),
        position_pda,
        requestargs.request_change,
        position_request_bump,
    )?;

    Ok(position_request_account)
}
