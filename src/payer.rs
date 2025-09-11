use base64::Engine;
use solana_sdk::{bs58, signature::Keypair};
use std::fs;

pub fn get_payer() -> Keypair {
    let wallet_base64 = fs::read_to_string("wallet.txt").expect("Failed to read");

    let decoded_bytes = base64::engine::general_purpose::STANDARD
        .decode(wallet_base64)
        .expect("Failed to decode base 64");
    let base58_wallet = bs58::encode(decoded_bytes).into_string();

    Keypair::from_base58_string(&base58_wallet)
}
