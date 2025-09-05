use borsh::{BorshDeserialize, BorshSerialize};
use solana_sdk::pubkey::Pubkey;

// Number of reward tokens
pub const REWARD_NUM: usize = 3;
pub const TICK_ARRAY_BITMAP_SIZE: usize = 16;

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct RewardInfo {
    /// Reward state: 0 = uninitialized, 1 = initialized
    pub reward_state: u8,
    /// Reward open time
    pub open_time: u64,
    /// Reward end time
    pub end_time: u64,
    /// Reward last update time
    pub last_update_time: u64,
    /// Q64.64 number indicating tokens per second per unit of liquidity
    pub emissions_per_second_x64: u128,
    /// Total amount of reward tokens emitted
    pub reward_total_emissioned: u64,
    /// Total amount of claimed reward tokens
    pub reward_claimed: u64,
    /// Reward token mint
    pub token_mint: Pubkey,
    /// Reward vault token account
    pub token_vault: Pubkey,
    /// Authority allowed to set reward parameters
    pub authority: Pubkey,
    /// Q64.64 number tracking total tokens earned per unit of liquidity
    pub reward_growth_global_x64: u128,
}

#[derive(BorshDeserialize, BorshSerialize, Debug)]
pub struct RaydiumPoolState {
    pub bump: [u8; 1],
    pub amm_config: Pubkey,
    pub owner: Pubkey,
    pub token_mint_0: Pubkey,
    pub token_mint_1: Pubkey,
    pub token_vault_0: Pubkey,
    pub token_vault_1: Pubkey,
    pub observation_key: Pubkey,
    pub mint_decimals_0: u8,
    pub mint_decimals_1: u8,
    pub tick_spacing: u16,
    pub liquidity: u128,
    pub sqrt_price_x64: u128,
    pub tick_current: i32,
    pub padding3: u16,
    pub padding4: u16,
    pub fee_growth_global_0_x64: u128,
    pub fee_growth_global_1_x64: u128,
    pub protocol_fees_token_0: u64,
    pub protocol_fees_token_1: u64,
    pub swap_in_amount_token_0: u128,
    pub swap_out_amount_token_1: u128,
    pub swap_in_amount_token_1: u128,
    pub swap_out_amount_token_0: u128,
    pub status: u8,
    pub padding: [u8; 7],
    pub reward_infos: [RewardInfo; REWARD_NUM],
    pub tick_array_bitmap: [u64; TICK_ARRAY_BITMAP_SIZE],
    pub total_fees_token_0: u64,
    pub total_fees_claimed_token_0: u64,
    pub total_fees_token_1: u64,
    pub total_fees_claimed_token_1: u64,
    pub fund_fees_token_0: u64,
    pub fund_fees_token_1: u64,
    pub open_time: u64,
    pub recent_epoch: u64,
    pub padding1: [u64; 24],
    pub padding2: [u64; 32],
}
