mod orca;
mod raydium;

use base64::engine::{self, general_purpose};

use base64::Engine;
use futures_util::StreamExt;
use solana_account_decoder::UiAccountEncoding;
use solana_client::nonblocking::pubsub_client::PubsubClient;
use solana_client::nonce_utils::get_account;
use solana_client::rpc_client::RpcClient;
use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::blake3::hash;
use solana_sdk::pubkey::Pubkey;
use std::io::Cursor;
use std::str::FromStr;
use std::sync::Arc;
use zstd::decode_all;

use borsh::BorshDeserialize;
use orca::Whirlpool;

use crate::raydium::RaydiumPoolState;

fn decode_orca(encoded: &str) -> Result<Whirlpool, Box<dyn std::error::Error>> {
    let bytes = general_purpose::STANDARD.decode(encoded).unwrap();

    let raw_bytes = decode_all(Cursor::new(bytes))?;
    let decoded = Whirlpool::try_from_slice(&raw_bytes)?;

    Ok(decoded)
}

fn decode_raydium(encoded: &str) -> Result<RaydiumPoolState, Box<dyn std::error::Error>> {
    let bytes = base64::Engine::decode(&general_purpose::STANDARD, encoded).unwrap();
    let raw_bytes = decode_all(Cursor::new(bytes))?;

    let data_without_discriminator = &raw_bytes[8..];

    let decoded = RaydiumPoolState::try_from_slice(&data_without_discriminator)?;

    Ok(decoded)
}

struct Data {
    pub sqrt_price: u128,
}

fn calculate_price_raw_B_per_a(data: &Data) -> f64 {
    let price_sqrt_root = (data.sqrt_price as f64) / (2_u128.pow(64) as f64);
    let price = (price_sqrt_root as f64).powi(2);

    price
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let ws_client_url =
        "wss://mainnet.helius-rpc.com/?api-key=5ddfdb35-09e4-48fb-8916-d57174620515";

    let ws_client = PubsubClient::new(ws_client_url).await?;
    let rpc_client = RpcClient::new(
        "https://mainnet.helius-rpc.com/?api-key=5ddfdb35-09e4-48fb-8916-d57174620515".to_string(),
    );

    let shared_ws_client = Arc::new(ws_client);

    let sol_usdc_pool_account_str = "Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE";
    let pool_account_pubkey = Pubkey::from_str(sol_usdc_pool_account_str)?;

    let config = RpcAccountInfoConfig {
        commitment: Some(CommitmentConfig::confirmed()),
        data_slice: None,
        encoding: Some(UiAccountEncoding::Base64Zstd),
        min_context_slot: None,
    };

    let raydium_usdc_pool_account =
        Pubkey::from_str("3ucNos4NbumPLZNWztqGHNFFgkHeRMBQAVemeeomsUxv")?;

    let clone1 = shared_ws_client.clone();
    let config1 = config.clone();

    tokio::spawn(async move {
        let (mut stream, _) = clone1
            .account_subscribe(&raydium_usdc_pool_account, Some(config1))
            .await
            .expect("Failed to stram");

        while let Some(account) = stream.next().await {
            match &account.value.data {
                solana_account_decoder::UiAccountData::Binary(encoded, _decoding) => {
                    match &decode_raydium(encoded) {
                        Ok(decoded) => {
                            let price = calculate_price_raw_B_per_a(&Data {
                                sqrt_price: decoded.sqrt_price_x64,
                            });
                            println!("{:#?}", price);
                        }
                        Err(err) => {
                            eprintln!("Error: {}", err)
                        }
                    }
                }
                solana_account_decoder::UiAccountData::LegacyBinary(encoded) => {
                    match &decode_raydium(encoded) {
                        Ok(decoded) => {
                            let price = calculate_price_raw_B_per_a(&Data {
                                sqrt_price: decoded.sqrt_price_x64,
                            });
                            println!("{:#?}", price)
                        }
                        Err(err) => {
                            eprintln!("Error: {}", err)
                        }
                    }
                }
                _ => {
                    eprintln!("Error")
                }
            }
        }
    });

    let clone2 = shared_ws_client.clone();
    let config2 = config.clone();
    tokio::spawn(async move {
        let (mut stream, _) = clone2
            .account_subscribe(&pool_account_pubkey, Some(config2))
            .await
            .expect("Failed stream");

        while let Some(account) = stream.next().await {
            match &account.value.data {
                solana_account_decoder::UiAccountData::LegacyBinary(encoded) => {
                    if let Ok(data) = decode_orca(encoded) {
                        let price = calculate_price_raw_B_per_a(&Data {
                            sqrt_price: data.sqrt_price,
                        });
                        // println!("{:#?}", data);
                        println!("Price: {}", price)
                    } else {
                        eprintln!("Error");
                    }
                }
                solana_account_decoder::UiAccountData::Binary(encoded, _decoding) => {
                    if let Ok(data) = decode_orca(encoded) {
                        let price = calculate_price_raw_B_per_a(&Data {
                            sqrt_price: data.sqrt_price,
                        });
                        // println!("{:#?}", data);
                        println!("Price: {}", price)
                    } else {
                        eprintln!("Error");
                    }
                }
                _ => {
                    eprintln!("Error")
                }
            };
        }
    });

    loop {}
}
