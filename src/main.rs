mod orca;

use std::str::FromStr;
use futures_util::StreamExt;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcAccountInfoConfig},
};
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{pubkey::Pubkey, signature::Keypair, signer::Signer};
use std::io::Cursor;
use zstd::decode_all;

use borsh::{BorshDeserialize};
use orca::{Whirlpool};

fn decode_orca(encoded: &str) -> Result<Whirlpool, Box<dyn std::error::Error>> {
    let bytes = base64::decode(encoded)?;
    let raw_bytes = decode_all(Cursor::new(bytes))?;
    let decoded = Whirlpool::try_from_slice(&raw_bytes)?;

    Ok(decoded)
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let ws_client_url = "wss://api.mainnet-beta.solana.com/";

    let ws_client = PubsubClient::new(ws_client_url).await?;

    let wallet = Keypair::new();
    let pubkey = wallet.pubkey();

    let sol_usdc_pool_account_str = "Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE";
    let pool_account_pubkey = Pubkey::from_str(sol_usdc_pool_account_str)?;

    let config = RpcAccountInfoConfig {
        commitment: Some(CommitmentConfig::confirmed()),
        data_slice: None,
        encoding: Some(UiAccountEncoding::Base64Zstd),
        min_context_slot: None,
    };

    tokio::spawn(async move {
        let (mut stream, _) = ws_client
            .account_subscribe(&pool_account_pubkey, Some(config.clone()))
            .await
            .expect("Failed stream");

        while let Some(account) = stream.next().await {
            match &account.value.data {
                solana_account_decoder::UiAccountData::LegacyBinary(encoded) => {
                    if let Ok(data) = decode_orca(encoded) {
                      println!("{:#?}", data);
                    } else {
                        eprintln!("Error");
                    }
                }
                solana_account_decoder::UiAccountData::Binary(encoded, _decoding) => {
                 if let Ok(data) = decode_orca(encoded) {
                    println!("{:#?}", data);
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
