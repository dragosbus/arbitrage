use solana_client::{client_error, rpc_client::RpcClient};
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{hash::Hash, pubkey::Pubkey, signer::Signer};
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_token::ID as TOKEN_PROGRAM_ID;
use std::str::FromStr;

use crate::payer::get_payer;

pub struct BotRpcClient {
    pub connection: RpcClient,
}

impl BotRpcClient {
    pub fn new(client_url: &str) -> Self {
        Self {
            connection: RpcClient::new(client_url),
        }
    }

    pub fn get_block_height(
        &self,
        commitment: Option<CommitmentConfig>,
    ) -> Result<u64, client_error::ClientError> {
        self.connection
            .get_block_height_with_commitment(commitment.unwrap_or(CommitmentConfig::confirmed()))
    }

    pub fn get_latest_block_hash(
        &self,
        commitment: Option<CommitmentConfig>,
    ) -> Result<(Hash, u64), client_error::ClientError> {
        self.connection.get_latest_blockhash_with_commitment(
            commitment.unwrap_or(CommitmentConfig::confirmed()),
        )
    }

    pub fn get_associated_token_account(&self, token_address: &str) -> Option<Pubkey> {
        let wallet = get_payer();

        match &Pubkey::from_str(token_address) {
            Ok(address) => {
                let account_data = get_associated_token_address_with_program_id(
                    &spl_associated_token_account::solana_program::pubkey::Pubkey::new_from_array(
                        wallet.pubkey().to_bytes(),
                    ),
                    &spl_associated_token_account::solana_program::pubkey::Pubkey::new_from_array(
                        address.to_bytes(),
                    ),
                    &TOKEN_PROGRAM_ID,
                );

                Some(Pubkey::new_from_array(account_data.to_bytes()))
            }
            Err(err) => {
                eprintln!("Error: {}", err);

                None
            }
        }
    }
}
