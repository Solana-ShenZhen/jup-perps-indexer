use std::env;

// use self::fee_estimation::CREATE_POOL_COMPUTE_UNIT;
use crate::jup_perps::accounts::custody;
use crate::jup_perps::accounts::perpetuals;
use crate::jup_perps::accounts::pool;
use crate::jup_perps::accounts::program_accounts;
use crate::jup_perps::accounts::Position;
use crate::jup_perps::accounts::PositionRequest;
use crate::jup_perps::instructions::create_increase_position_market_request;
use crate::jup_perps::instructions::CreateIncreasePositionMarketRequest;
use crate::jup_perps::types::CreateIncreasePositionMarketRequestParams;
use crate::rust_client::transaction_utils::create_ata;
use crate::rust_client::types::FindPositionPdaArgs;
use crate::rust_client::types::FindRequestPdaArgs;
use crate::rust_client::types::OpenPositionArgs;
use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::transaction::Transaction;

// entrypoint!(open_position);

pub fn open_position(args: &OpenPositionArgs) -> Result<(), Box<dyn std::error::Error>> {
    let owner = args.owner;

    let position_pda = create_position_account(FindPositionPdaArgs {
        custody: args.custody,
        collateral_custody: args.collateral_custody,
        owner: args.owner,
        side: args.side,
    })
    .unwrap();

    let position_request_pda = create_position_request_account(
        FindRequestPdaArgs {
            counter: args.counter,
            position_pubkey: position_pda,
            request_change: args.request_change,
        },
        FindPositionPdaArgs {
            custody: args.custody,
            collateral_custody: args.collateral_custody,
            owner: args.owner,
            side: args.side,
        },
    )
    .unwrap();

    let funding_account = create_ata(&args.owner, &args.input_mint);

    let perpetuals = perpetuals::Perpetuals::ADDRESS;

    let pool = pool::Pool::ADDRESS;

    let position_request_ata = create_ata(&position_request_pda, &args.input_mint);

    let custody = match args.input_mint.to_string().as_str() {
        "So11111111111111111111111111111111111111112" => custody::Custody::SOL_ADDRESS,
        "7vfCXTUXx5WJV5JADk17DUJ4ksgau7utNKj4b963voxs" => custody::Custody::ETH_ADDRESS,
        "3NZ9JMVBmGAqocybic2c7LQCJScmgsAZ6vQqTDzcqmJh" => custody::Custody::BTC_ADDRESS,
        "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v" => custody::Custody::USDC_ADDRESS,
        "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB" => custody::Custody::USDT_ADDRESS,
        _ => return Err("Unsupported input mint".into()),
    };

    let collateral_custody = custody.clone();

    let input_mint = args.input_mint;

    let referral = program_accounts::REFERRER_ADDRESS;

    let token_program = program_accounts::TOKEN_PROGRAM_ADDRESS;

    let associated_token_program = program_accounts::ASSOCIATED_TOKEN_PROGRAM_ADDRESS;

    let system_program = program_accounts::SYSTEM_PROGRAM_ADDRESS;

    let event_authority = program_accounts::EVENT_AUTHORITY_ADDRESS;

    let program = program_accounts::PROGRAM_ADDRESS;

    let request = CreateIncreasePositionMarketRequest {
        owner,
        funding_account,
        perpetuals,
        pool,
        position: position_pda,
        position_request: position_request_pda,
        position_request_ata,
        custody,
        collateral_custody,
        input_mint,
        referral: Some(referral),
        token_program,
        associated_token_program,
        system_program,
        event_authority,
        program,
    };

    let params = CreateIncreasePositionMarketRequestParams {
        size_usd_delta: args.size_usd_delta,
        collateral_token_delta: args.collateral_token_delta,
        side: args.side,
        price_slippage: args.price_slippage,
        jupiter_minimum_out: None,
        counter: args.counter.unwrap(),
    };

    let instruction = request.instruction(
        create_increase_position_market_request::CreateIncreasePositionMarketRequestInstructionArgs {
            params
        }
    );

    dotenv().ok();

    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not found in .env");
    let keypair = Keypair::from_base58_string(&private_key);

    let rpc_url =
        "https://mainnet.helius-rpc.com/?api-key=f46e7c57-a4d4-43b0-b65b-1f287e2380cb".to_string();
    let client = RpcClient::new(rpc_url);

    let recent_blockhash = client.get_latest_blockhash()?;
    let transaction = Transaction::new_signed_with_payer(
        &[instruction],
        Some(&keypair.pubkey()),
        &[&keypair],
        recent_blockhash,
    );

    // 发送交易
    let signature = client.send_and_confirm_transaction(&transaction)?;
    println!("Transaction sent: {}", signature);

    Ok(())
}

pub fn create_position_account(
    args: FindPositionPdaArgs,
) -> Result<Pubkey, Box<dyn std::error::Error>> {
    let (position_pda, bump) =
        Position::find_pda(args.custody, args.collateral_custody, args.owner, args.side);

    Position::create_pda(
        args.custody,
        args.collateral_custody,
        args.owner,
        args.side,
        bump,
    )?;

    Ok(position_pda)
}

pub fn create_position_request_account(
    requestargs: FindRequestPdaArgs,
    positionargs: FindPositionPdaArgs,
) -> Result<Pubkey, Box<dyn std::error::Error>> {
    let (position_pda, _position_bump) = Position::find_pda(
        positionargs.custody,
        positionargs.collateral_custody,
        positionargs.owner,
        positionargs.side,
    );

    let (_position_request_pda, position_request_bump) = PositionRequest::find_pda(
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
