use solana_program::pubkey::Pubkey;
use spl_associated_token_account::get_associated_token_address;

pub fn create_ata(wallet_address: &Pubkey, mint_address: &Pubkey) -> Pubkey {
    get_associated_token_address(wallet_address, mint_address)
}
