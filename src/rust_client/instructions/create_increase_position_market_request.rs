use std::env;

// use self::fee_estimation::CREATE_POOL_COMPUTE_UNIT;
use crate::jup_perps::instructions::create_increase_position_market_request;
use crate::jup_perps::instructions::CreateIncreasePositionMarketRequest;
use crate::jup_perps::types::CreateIncreasePositionMarketRequestParams;
use crate::rust_client::types::OpenPositionArgs;
use anchor_spl::token::spl_token;
use dotenv::dotenv;
use solana_client::rpc_client::RpcClient;
use solana_program::pubkey;
use solana_sdk::account::Account;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::system_instruction;
use solana_sdk::transaction::Transaction;
use spl_associated_token_account::get_associated_token_address;
use spl_associated_token_account::instruction::create_associated_token_account;
use spl_token::instruction::sync_native;
use spl_token::state::Account as TokenAccount;

pub fn create_wsol_ata(owner: Pubkey) -> solana_sdk::instruction::Instruction {
    let wsol_mint = pubkey!("So11111111111111111111111111111111111111112");

    create_associated_token_account(&owner, &owner, &wsol_mint, &spl_token::id())
}

pub fn transfer_wsol(
    owner: Pubkey,
    to: Pubkey,
    amount: u64,
) -> solana_sdk::instruction::Instruction {
    system_instruction::transfer(&owner, &to, amount)
}

pub fn sync_wsol(wsol_ata: Pubkey) -> solana_sdk::instruction::Instruction {
    sync_native(&spl_token::id(), &wsol_ata).unwrap()
}

pub fn open_position(args: &OpenPositionArgs) -> Result<(), Box<dyn std::error::Error>> {
    let request = CreateIncreasePositionMarketRequest {
        owner: args.owner,
        funding_account: args.funding_account,
        perpetuals: args.perpetuals,
        pool: args.pool,
        position: args.position,
        position_request: args.position_request,
        position_request_ata: args.position_request_ata,
        custody: args.custody,
        collateral_custody: args.collateral_custody,
        input_mint: args.input_mint,
        referral: Some(args.referral),
        token_program: args.token_program,
        associated_token_program: args.associated_token_program,
        system_program: args.system_program,
        event_authority: args.event_authority,
        program: args.program,
    };

    let params = CreateIncreasePositionMarketRequestParams {
        size_usd_delta: args.size_usd_delta,
        collateral_token_delta: args.collateral_token_delta,
        side: args.side,
        price_slippage: args.price_slippage,
        jupiter_minimum_out: None,
        counter: args.counter,
    };

    let create_position = request.instruction(
        create_increase_position_market_request::CreateIncreasePositionMarketRequestInstructionArgs {
            params
        }
    );

    dotenv().ok();

    let private_key = env::var("PRIVATE_KEY").expect("PRIVATE_KEY not found in .env");
    let keypair = Keypair::from_base58_string(&private_key);

    let rpc_url =
        "https://mainnet.helius-rpc.com/?api-key=b85b3e7d-9471-4b9a-9f94-b1fb9420449c".to_string();
    let client = RpcClient::new(rpc_url);

    // 根据不同的input_mint 帮忙创建不同的ata

    let recent_blockhash = client.get_latest_blockhash()?;

    let wsol_mint = pubkey!("So11111111111111111111111111111111111111112");
    let wsol_ata = get_associated_token_address(&args.owner, &wsol_mint);

    let mut instructions = vec![];
    match client.get_account(&wsol_ata) {
        Ok(_) => {
            println!("WSOL ATA already exists");
        }
        Err(_) => {

            println!("Creating WSOL ATA");
            instructions.push(create_wsol_ata(args.owner));
            instructions.push(transfer_wsol(args.owner, args.funding_account, args.paying));
            instructions.push(sync_wsol(args.funding_account));
        }
    }

    instructions.push(create_position);

    let transaction = Transaction::new_signed_with_payer(
        &instructions,
        Some(&keypair.pubkey()),
        &[&keypair],
        recent_blockhash,
    );

    // simulate transaction
    let simulation = client.simulate_transaction(&transaction)?;

    println!("Simulation result: {:#?}", simulation.value);

    // let signature = client.send_and_confirm_transaction(&transaction)?;
    // println!("Transaction sent: {}", signature);

    Ok(())
}
