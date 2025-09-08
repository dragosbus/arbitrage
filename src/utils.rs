use crate::pools_struct::orca::Whirlpool;
use crate::pools_struct::raydium::RaydiumPoolState;
use base64::engine::{self, general_purpose};
use base64::Engine;
use borsh::BorshDeserialize;
use std::io::Cursor;
use zstd::decode_all;

pub struct Data {
    pub sqrt_price: u128,
}

pub fn decode_orca(encoded: &str) -> Result<Whirlpool, Box<dyn std::error::Error>> {
    let bytes = general_purpose::STANDARD.decode(encoded).unwrap();

    let raw_bytes = decode_all(Cursor::new(bytes))?;
    let decoded = Whirlpool::try_from_slice(&raw_bytes)?;

    Ok(decoded)
}

pub fn decode_raydium(encoded: &str) -> Result<RaydiumPoolState, Box<dyn std::error::Error>> {
    let bytes = base64::Engine::decode(&general_purpose::STANDARD, encoded).unwrap();
    let raw_bytes = decode_all(Cursor::new(bytes))?;

    let data_without_discriminator = &raw_bytes[8..];

    let decoded = RaydiumPoolState::try_from_slice(&data_without_discriminator)?;

    Ok(decoded)
}

pub fn calculate_price_raw_B_per_a(data: &Data) -> f64 {
    let price_sqrt_root = (data.sqrt_price as f64) / (2_u128.pow(64) as f64);
    let price = (price_sqrt_root as f64).powi(2);

    price
}
