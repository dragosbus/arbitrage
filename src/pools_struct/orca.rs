use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

use crate::pools_struct::{error::PoolError, structs::PriceFetcher};

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct RewardInfo {
    pub mint: Pubkey,
    pub vault: Pubkey,
    pub extension: [u8; 32],
    pub emissions_per_second_x64: u128,
    pub growth_global_x64: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct WhirlpoolState {
    pub discriminator: [u8; 8],
    pub whirlpools_config: Pubkey,
    pub whirlpool_bump: u8,
    pub tick_spacing: u16,
    pub fee_tier_index_seed: [u8; 2],
    pub fee_rate: u16,
    pub protocol_fee_rate: u16,
    pub liquidity: u128,
    pub sqrt_price: u128,
    pub tick_current_index: i32,
    pub protocol_fee_owed_a: u64,
    pub protocol_fee_owed_b: u64,
    pub token_mint_a: Pubkey,
    pub token_vault_a: Pubkey,
    pub fee_growth_global_a: u128,
    pub token_mint_b: Pubkey,
    pub token_vault_b: Pubkey,
    pub fee_growth_global_b: u128,
    pub reward_last_updated_timestamp: u64,
    pub reward_infos: [RewardInfo; 3], // adjust NUM_REWARDS
}

impl WhirlpoolState {
    pub fn calculate_price(&self) -> Result<f64, PoolError> {
        if self.sqrt_price == 0 {
            return Err(PoolError::PriceCalculationFailed.into());
        }
        let price_sqrt_root = (self.sqrt_price as f64) / (2_u128.pow(64) as f64);
        let price = (price_sqrt_root as f64).powi(2);

        Ok(price)
    }
}

impl PriceFetcher for WhirlpoolState {
    fn get_dex_name(&self) -> &'static str {
        "orca"
    }

    fn get_price(&self) -> Result<f64, PoolError> {
        self.calculate_price()
    }
}
