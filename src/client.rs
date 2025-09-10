use std::str::FromStr;

use base64::Engine;
use solana_client::{client_error, rpc_client::RpcClient};
use solana_commitment_config::CommitmentConfig;
use solana_sdk::{bs58, hash::Hash, signature::Keypair, signer::Signer};
use spl_associated_token_account::get_associated_token_address_with_program_id;
use spl_associated_token_account::solana_program::pubkey::Pubkey;
use spl_token::ID as TOKEN_PROGRAM_ID;

pub struct BotRpcClient {
    connection: RpcClient,
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
        let wallet_base64 = "EaH2YKbSYyx3lpXLtlAOBwvpGdW/mS/u94ngJLcexAu8ltpoe8WGH8JKWwttgEN869h5B4glZ9I2YihVEQd9zQ==";
        let decoded_bytes = base64::engine::general_purpose::STANDARD
            .decode(wallet_base64)
            .expect("Failed to decode base 64");
        let base58_wallet = bs58::encode(decoded_bytes).into_string();

        let wallet = Keypair::from_base58_string(&base58_wallet);

        match &Pubkey::from_str(token_address) {
            Ok(address) => {
                let account_data = get_associated_token_address_with_program_id(
                    &Pubkey::new_from_array(wallet.pubkey().to_bytes()),
                    &address,
                    &TOKEN_PROGRAM_ID,
                );

                Some(account_data)
            }
            Err(err) => {
                eprintln!("Error: {}", err);

                None
            }
        }
    }
}
