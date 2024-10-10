use base64;
use bs58;
use sha2::{Digest, Sha256};
use std::convert::TryInto;

pub fn main() {
    let base58_encoded = "28FxtWhjvWAAWhqsv4U5oTMCFGWXfMBLwk1mWHcidb1K7VpEQfw9MrXKmPvhZfFuVcd1mn2g1tfjHoNu3H7kG5G3ZBi18muvc7JEzmGY1CUBrbzorkVRkYr22DD7dnqbCVnKQZiExeUQMU6J9xKS36wQmwkn9QvZ8rzJxfo7o9aagoX3cRDmt5QpvVzYp2GeXRxm9a7e78TmYnEV41PFJBAxfEQvwFARM9rrGvQTA9fAzDQD4B1xVy8aRhvSXHJ5PHYtrX7Eh76VDiZED4GDEeH93pveWzxN6owqunMCRA9h3zryzXK198KfAoWfPVQrpJDXKVkaNGF5afxgiWKCCAtUcK8pahr8KkL7yqsrXxRpdruatpTXNssRcvzyegrwDZG9AWvUw6mobWXwG5pK8aWfZqe3BRYzLHXX6vWmvSWt7uVXu1zEP8oURAsU7SPGFeJjj2pf"; 

    // Given: event_str and base64_encoded_data
    let event_str = "event:CreatePositionRequestEvent";
    let base64_encoded_data = bs58::decode(base58_encoded).into_vec().unwrap();// Replace with your actual Base64 string

    println!("base64_encoded_data: {:?}", base64_encoded_data);
    // 1. Calculate the discriminator (first 8 bytes)
    let mut hasher = Sha256::new();
    hasher.update(event_str.as_bytes());
    let result = hasher.finalize();
    let discriminator = &result[..8]; // 事件判别器的前8字节

    // 2. Decode Base64
    let decoded_data = base64::decode(base64_encoded_data).unwrap();

    // Verify the discriminator matches the first 8 bytes
    if &decoded_data[..8] != discriminator {
        println!("Discriminator does not match!");
        return;
    }

    println!("Discriminator matches!");

    // 3. Parse each field according to its type

    // Offsets to track where each field starts
    let mut offset = 8; // Discriminator is the first 8 bytes

    // PublicKeys are 32 bytes each
    let owner = &decoded_data[offset..offset + 32];
    println!("Owner: {}", encode_base58(owner));
    offset += 32;

    let pool = &decoded_data[offset..offset + 32];
    println!("Pool: {}", encode_base58(pool));
    offset += 32;

    let position_key = &decoded_data[offset..offset + 32];
    println!("Position Key: {}", encode_base58(position_key));
    offset += 32;

    // u8 (1 byte)
    let position_side = decoded_data[offset];
    println!("Position Side: {}", position_side);
    offset += 1;

    let position_mint = &decoded_data[offset..offset + 32];
    println!("Position Mint: {}", encode_base58(position_mint));
    offset += 32;

    let position_custody = &decoded_data[offset..offset + 32];
    println!("Position Custody: {}", encode_base58(position_custody));
    offset += 32;

    let position_collateral_mint = &decoded_data[offset..offset + 32];
    println!(
        "Position Collateral Mint: {}",
        encode_base58(position_collateral_mint)
    );
    offset += 32;

    let position_collateral_custody = &decoded_data[offset..offset + 32];
    println!(
        "Position Collateral Custody: {}",
        encode_base58(position_collateral_custody)
    );
    offset += 32;

    let position_request_key = &decoded_data[offset..offset + 32];
    println!(
        "Position Request Key: {}",
        encode_base58(position_request_key)
    );
    offset += 32;

    let position_request_mint = &decoded_data[offset..offset + 32];
    println!(
        "Position Request Mint: {}",
        encode_base58(position_request_mint)
    );
    offset += 32;

    // u64 fields (8 bytes each)
    let size_usd_delta = u64::from_le_bytes(decoded_data[offset..offset + 8].try_into().unwrap());
    println!("Size USD Delta: {}", size_usd_delta);
    offset += 8;

    let collateral_delta = u64::from_le_bytes(decoded_data[offset..offset + 8].try_into().unwrap());
    println!("Collateral Delta: {}", collateral_delta);
    offset += 8;

    // Option<u64> (1 byte for Some/None, followed by 8 bytes if Some)
    let (price_slippage, price_slippage_size) = decode_option_u64(&decoded_data[offset..]);
    println!("Price Slippage: {:?}", price_slippage);
    offset += price_slippage_size;

    let (jupiter_minimum_out, jupiter_minimum_out_size) =
        decode_option_u64(&decoded_data[offset..]);
    println!("Jupiter Minimum Out: {:?}", jupiter_minimum_out);
    offset += jupiter_minimum_out_size;

    let (pre_swap_amount, pre_swap_amount_size) = decode_option_u64(&decoded_data[offset..]);
    println!("Pre Swap Amount: {:?}", pre_swap_amount);
    offset += pre_swap_amount_size;

    // u8
    let request_change = decoded_data[offset];
    println!("Request Change: {}", request_change);
    offset += 1;

    // i64 (8 bytes)
    let open_time = i64::from_le_bytes(decoded_data[offset..offset + 8].try_into().unwrap());
    println!("Open Time: {}", open_time);
    offset += 8;
}

// Helper function to decode Base58 from bytes
fn encode_base58(bytes: &[u8]) -> String {
    bs58::encode(bytes).into_string()
}

// Helper function to decode Option<u64>
fn decode_option_u64(data: &[u8]) -> (Option<u64>, usize) {
    if data[0] == 0 {
        (None, 1) // 1 byte for None
    } else {
        let value = u64::from_le_bytes(data[1..9].try_into().unwrap());
        (Some(value), 9) // 1 byte for Some + 8 bytes for u64 value
    }
}
