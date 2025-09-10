mod client;
mod pools_struct;
mod utils;

use dashmap::DashMap;
use futures_util::StreamExt;
use solana_account_decoder::UiAccountEncoding;
use solana_client::nonblocking::pubsub_client::PubsubClient;
use solana_client::rpc_config::RpcAccountInfoConfig;
use solana_commitment_config::CommitmentConfig;
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use std::sync::Arc;
use utils::{calculate_price_raw_B_per_a, decode_orca, decode_raydium, Data};

use crate::client::BotRpcClient;

#[derive(Debug)]
struct Price {
    price: f64,
}

#[derive(Clone)]
struct Pool {
    name: String,
    pool_id: Pubkey,
}

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let ws_client_url =
        "wss://mainnet.helius-rpc.com/?api-key=5ddfdb35-09e4-48fb-8916-d57174620515";

    let ws_client = Arc::new(PubsubClient::new(ws_client_url).await?);
    let rpc_client = BotRpcClient::new(
        "https://mainnet.helius-rpc.com/?api-key=5ddfdb35-09e4-48fb-8916-d57174620515",
    );

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

    let addresses = [
        Pool {
            name: "orca".to_string(),
            pool_id: pool_account_pubkey,
        },
        Pool {
            name: "raydium".to_string(),
            pool_id: raydium_usdc_pool_account,
        },
    ];

    // loop {
    //     let multiple_accounts = rpc_client
    //         .get_multiple_accounts_with_commitment(&addresses, config.commitment.unwrap());

    //     match multiple_accounts {
    //         Ok(data) => {
    //             let v = &data.value;

    //             let wirpool_data = v[0].as_ref().unwrap().data.clone();
    //             let raydium_data = v[1].as_ref().unwrap().data.clone();

    //             let w = Whirlpool::try_from_slice(&wirpool_data);

    //             let w_price = calculate_price_raw_B_per_a(&Data {
    //                 sqrt_price: w.unwrap().sqrt_price,
    //             });

    //             let r = RaydiumPoolState::try_from_slice(&raydium_data[8..]);

    //             let r_price = calculate_price_raw_B_per_a(&Data {
    //                 sqrt_price: r.unwrap().sqrt_price_x64,
    //             });

    //             println!("ORCA: {:#?}", w_price);
    //             println!("Raydium: {:#?}", r_price);

    //             // for (i, elem) in data.value.iter().enumerate() {
    //             //     let w = ele
    //             //     // println!("{:#?}", elem.as_ref().unwrap().data);
    //             //     println!("{:#?}", elem.as_ref().unwrap());
    //             // }
    //         }
    //         Err(err) => {
    //             eprintln!("ERROR: {}", err)
    //         }
    //     };
    // }

    let markets = Arc::new(DashMap::<String, Price>::new());

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    for pool in addresses.iter() {
        let markets = markets.clone();
        let ws_client_clone = Arc::clone(&ws_client);
        let config_clone = config.clone();
        let pool = pool.clone();

        let tx = tx.clone();

        tokio::spawn(async move {
            let (mut stream, _) = ws_client_clone
                .account_subscribe(&pool.pool_id, Some(config_clone))
                .await
                .expect("Failed to stream");

            while let Some(account) = stream.next().await {
                match &account.value.data {
                    solana_account_decoder::UiAccountData::Binary(encoded, _)
                    | solana_account_decoder::UiAccountData::LegacyBinary(encoded) => {
                        let pool_name = pool.name.clone();

                        match pool_name.as_str() {
                            "orca" => match &decode_orca(encoded) {
                                Ok(decoded) => {
                                    let price = calculate_price_raw_B_per_a(&Data {
                                        sqrt_price: decoded.sqrt_price,
                                    });
                                    markets.insert(pool_name.clone(), Price { price: price });
                                    tx.send((pool_name.clone(), Price { price: price }))
                                        .unwrap();
                                }
                                Err(err) => {
                                    eprintln!("Error: {}", err)
                                }
                            },
                            "raydium" => match &decode_raydium(encoded) {
                                Ok(decoded) => {
                                    let price = calculate_price_raw_B_per_a(&Data {
                                        sqrt_price: decoded.sqrt_price_x64,
                                    });
                                    markets.insert(pool_name.clone(), Price { price: price });
                                    tx.send((pool_name.clone(), Price { price: price }))
                                        .unwrap();
                                }
                                Err(err) => {
                                    eprintln!("Error:{}", err)
                                }
                            },
                            _ => {
                                eprintln!("Unknown pool name: {}", pool_name);
                            }
                        };
                    }
                    _ => {
                        eprintln!("Error")
                    }
                }
            }
        });
    }

    // tokio::spawn(async move {
    //     loop {
    //         for entry in markets.iter() {
    //             println!("{:#?}: {:#?}", entry.key(), entry.price);
    //         }
    //         sleep(Duration::from_millis(200)).await
    //     }
    // });

    tokio::spawn(async move {
        while let Some((pool_name, price)) = rx.recv().await {
            // println!("Updated {}: {:#?}", pool_name, price);

            let v = markets.get(pool_name.as_str()).unwrap();

            println!("{:#?}", v.price);
        }
    });

    tokio::signal::ctrl_c().await?;

    Ok(())
}
