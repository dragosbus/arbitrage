use std::str::FromStr;

use futures_util::StreamExt;
use solana_account_decoder::UiAccountEncoding;
use solana_client::{
    nonblocking::pubsub_client::PubsubClient,
    rpc_config::{RpcAccountInfoConfig, RpcProgramAccountsConfig},
};
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{
    account, borsh1, bs58::decode, log, message, pubkey::Pubkey, signature::Keypair, signer::Signer,
};
use std::io::Cursor;
use zstd::decode_all;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let ws_client_url = "wss://api.mainnet-beta.solana.com/";

    let ws_client = PubsubClient::new(ws_client_url).await?;

    let wallet = Keypair::new();
    let pubkey = wallet.pubkey();

    let program_id = "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc";
    let program_pub_key = Pubkey::from_str(program_id)?;

    let sol_usdc_pool_account_str = "Czfq3xZZDmsdGdUyrNLtRhGc47cXcZtLG4crryfu44zE";
    let pool_account_pubkey = Pubkey::from_str(sol_usdc_pool_account_str)?;

    let config = RpcAccountInfoConfig {
        commitment: Some(CommitmentConfig::confirmed()),
        data_slice: None,
        encoding: Some(UiAccountEncoding::Base64Zstd),
        min_context_slot: None,
    };

    let program_config = RpcProgramAccountsConfig {
        account_config: config.clone(),
        filters: None,
        sort_results: None,
        with_context: None,
    };

    tokio::spawn(async move {
        let (mut stream, mut receiver) = ws_client
            .account_subscribe(&pool_account_pubkey, Some(config.clone()))
            .await
            .expect("Failed stream");

        while let Some(account) = stream.next().await {
            match &account.value.data {
                solana_account_decoder::UiAccountData::LegacyBinary(encoded) => {
                    let bytes = base64::decode(encoded).unwrap();
                    println!("{:#?}", bytes);
                }
                solana_account_decoder::UiAccountData::Binary(encoded, _decoding) => {
                    let bytes = base64::decode(encoded).unwrap();
                    println!("{:#?}", bytes);
                }
                _ => {
                    eprintln!("Error")
                }
            };
        }

        let (mut stream, _) = ws_client
            .program_subscribe(&program_pub_key, Some(program_config.clone()))
            .await
            .expect("Failed");

        while let Some(message) = stream.next().await {
            // message.value.account.data is usually a tuple: (base64_string, encoding)
            let (encoded, _encoding) = match &message.value.account.data {
                solana_account_decoder::UiAccountData::Binary(data, _encoding) => (data, _encoding),
                _ => continue, // skip if not binary
            };

            // Decode base64
            let compressed_bytes = match base64::decode(encoded) {
                Ok(bytes) => bytes,
                Err(e) => {
                    eprintln!("base64 decode error");
                    continue;
                }
            };
            // Decompress zstd
            let raw_bytes = match decode_all(Cursor::new(compressed_bytes)) {
                Ok(bytes) => bytes,
                Err(e) => {
                    eprintln!("zstd decode error");
                    continue;
                }
            };
            // println!("{:#?}", raw_bytes);
        }
    });

    loop {}
}
