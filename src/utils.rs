use crate::pools_struct::orca::Whirlpool;
use crate::pools_struct::raydium::RaydiumPoolState;
use base64::engine::{self, general_purpose};
use base64::Engine;
use borsh::BorshDeserialize;
use spl_token::solana_program::instruction::Instruction;
use spl_token::solana_program::pubkey::Pubkey;
use std::io::Cursor;
use std::str::FromStr;
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

pub fn set_compute_unit_limit(units: u32) -> Instruction {
    let data = [
        2u8,                  // Instruction index for SetComputeUnitLimit (from Solscan: 02)
        (units & 0xFF) as u8, // Little-endian u32
        ((units >> 8) & 0xFF) as u8,
        ((units >> 16) & 0xFF) as u8,
        ((units >> 24) & 0xFF) as u8,
    ]
    .to_vec();
    Instruction {
        program_id: Pubkey::from_str("ComputeBudget111111111111111111111111111111").unwrap(),
        accounts: vec![],
        data,
    }
}

pub fn set_compute_unit_price(micro_lamports: u64) -> Instruction {
    let data = [
        3u8, // Assuming next index for SetComputeUnitPrice
        (micro_lamports & 0xFF) as u8,
        ((micro_lamports >> 8) & 0xFF) as u8,
        ((micro_lamports >> 16) & 0xFF) as u8,
        ((micro_lamports >> 24) & 0xFF) as u8,
        ((micro_lamports >> 32) & 0xFF) as u8,
        ((micro_lamports >> 40) & 0xFF) as u8,
        ((micro_lamports >> 48) & 0xFF) as u8,
        ((micro_lamports >> 56) & 0xFF) as u8,
    ]
    .to_vec();
    Instruction {
        program_id: Pubkey::from_str("ComputeBudget111111111111111111111111111111").unwrap(),
        accounts: vec![],
        data,
    }
}
