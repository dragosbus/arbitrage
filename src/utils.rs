use crate::pools_struct::error::PoolError;
use crate::pools_struct::meteora::MeteoraPoolState;
use crate::pools_struct::orca::WhirlpoolState;
use crate::pools_struct::raydium::RaydiumPoolState;
use crate::pools_struct::structs::DexType;
use crate::pools_struct::structs::PriceFetcher;
use base64::engine::{self, general_purpose};
use base64::Engine;
use borsh::BorshDeserialize;
use solana_sdk::message::Instruction;
use solana_sdk::pubkey::Pubkey;
use std::io::Cursor;
use std::str::FromStr;
use zstd::decode_all;

pub fn parse_encoded_data(encoded: &str, dex: DexType) -> Result<Box<dyn PriceFetcher>, PoolError> {
    match dex {
        DexType::Meteora => {
            let bytes = base64::Engine::decode(&general_purpose::STANDARD, encoded).unwrap();
            let raw_bytes = decode_all(Cursor::new(bytes)).expect("faled decoind all bytes");

            let data_without_discriminator = &raw_bytes[8..];

            let decoded = MeteoraPoolState::try_from_slice(&data_without_discriminator)
                .expect("failed decoding pool state");

            Ok(Box::new(decoded))
        }
        DexType::Raydium => {
            let bytes = base64::Engine::decode(&general_purpose::STANDARD, encoded).unwrap();
            let raw_bytes = decode_all(Cursor::new(bytes)).expect("faled decoind all bytes");

            let data_without_discriminator = &raw_bytes[8..];

            let decoded = RaydiumPoolState::try_from_slice(&data_without_discriminator)
                .expect("failed decoding pool state");

            Ok(Box::new(decoded))
        }
        DexType::Orca => {
            let bytes = general_purpose::STANDARD.decode(encoded).unwrap();

            let raw_bytes = decode_all(Cursor::new(bytes)).expect("faled decoind all bytes");
            let decoded =
                WhirlpoolState::try_from_slice(&raw_bytes).expect("failed decoding pool state");

            Ok(Box::new(decoded))
        }
        // DexType::HumidiFi => {
        //     let price = 0f64;

        //     Ok(price)
        // }
        // DexType::Lifinity => {
        //     let price = 0f64;

        //     Ok(price)
        // }
        // DexType::PancakeSwap => {
        //     let price = 0f64;

        //     Ok(price)
        // }
        // DexType::SolFiV2 => {
        //     let price = 0f64;

        //     Ok(price)
        // }
        _ => {
            eprintln!("Unknown pool type");
            Err(PoolError::InvalidPoolData.into())
        }
    }
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
