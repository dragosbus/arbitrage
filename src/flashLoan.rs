use sha2::{Digest, Sha256};
use solana_sdk::sysvar::ID as SysVarID;
use spl_token::solana_program::instruction::{AccountMeta, Instruction};
use spl_token::solana_program::pubkey::Pubkey;
use spl_token::ID as TOKEN_PROGRAM_ID;
use std::str::FromStr;

pub fn borrow_instruction_builder(
    payer: Pubkey,
    user_token_account: Pubkey,
    liquidity: u64,
) -> Instruction {
    let accounts = FlashBorrowAccounts {
        user_transfer_authority: payer,
        lending_market_authority: Pubkey::from_str("Dx8iy2o46sK1DzWbEcznqSKeLbLVeu7otkibA3WohGAj")
            .unwrap(),
        lending_market: Pubkey::from_str("H6rHXmXoCQvq8Ue81MqNh7ow5ysPa1dSozwW3PU1dDH6").unwrap(),
        reserve: Pubkey::from_str("6gTJfuPHEg6uRAijRkMqNc9kan4sVZejKMxmvx2grT1p").unwrap(),
        reserve_liquidity_mint: Pubkey::from_str("So11111111111111111111111111111111111111112")
            .unwrap(),
        reserve_source_liquidity: Pubkey::from_str("ywaaLvG7t1vXJo8sT3UzE8yzzZtxLM7Fmev64Jbooye")
            .unwrap(),
        user_destination_liquidity: user_token_account,
        reserve_liquidity_fee_receiver: Pubkey::from_str(
            "EQ7hw63aBS7aPQqXsoxaaBxiwbEzaAiY9Js6tCekkqxf",
        )
        .unwrap(),
        referrer_token_state: Pubkey::from_str("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD")
            .unwrap(),
        referrer_account: Pubkey::from_str("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD").unwrap(),
        sysvar_info: Pubkey::new_from_array(SysVarID.to_bytes()), // SysvarRent111111111111111111111111111111111
        token_program: TOKEN_PROGRAM_ID,
    };

    let mut hasher = Sha256::new();
    hasher.update(b"global:flashBorrowReserveLiquidity");
    let hash = hasher.finalize();
    let discriminator = &hash[..8];

    let mut data = Vec::with_capacity(16);
    data.extend_from_slice(discriminator);
    data.extend_from_slice(&liquidity.to_le_bytes());

    Instruction {
        program_id: Pubkey::from_str("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD").unwrap(),
        accounts: vec![
            AccountMeta::new_readonly(accounts.user_transfer_authority, true),
            AccountMeta::new_readonly(accounts.lending_market_authority, false),
            AccountMeta::new_readonly(accounts.lending_market, false),
            AccountMeta::new(accounts.reserve, false),
            AccountMeta::new_readonly(accounts.reserve_liquidity_mint, false),
            AccountMeta::new(accounts.reserve_source_liquidity, false),
            AccountMeta::new(accounts.user_destination_liquidity, false),
            AccountMeta::new(accounts.reserve_liquidity_fee_receiver, false),
            AccountMeta::new(accounts.referrer_token_state, false),
            AccountMeta::new(accounts.referrer_account, false),
            AccountMeta::new_readonly(accounts.sysvar_info, false),
            AccountMeta::new_readonly(accounts.token_program, false),
        ],
        data: data,
    }
}

pub fn repay_instruction_builder(
    payer: Pubkey,
    liquidity: u64,
    user_token_account: Pubkey,
    borrow_index: u16,
) -> Instruction {
    let accounts = FlashRepayAccounts {
        user_transfer_authority: payer,
        lending_market_authority: Pubkey::from_str("Dx8iy2o46sK1DzWbEcznqSKeLbLVeu7otkibA3WohGAj")
            .unwrap(),
        lending_market: Pubkey::from_str("H6rHXmXoCQvq8Ue81MqNh7ow5ysPa1dSozwW3PU1dDH6").unwrap(),
        reserve: Pubkey::from_str("6gTJfuPHEg6uRAijRkMqNc9kan4sVZejKMxmvx2grT1p").unwrap(),
        reserve_liquidity_mint: Pubkey::from_str("So11111111111111111111111111111111111111112")
            .unwrap(),
        reserve_destination_liquidity: Pubkey::from_str(
            "ywaaLvG7t1vXJo8sT3UzE8yzzZtxLM7Fmev64Jbooye",
        )
        .unwrap(),
        user_source_liquidity: user_token_account,
        reserve_liquidity_fee_receiver: Pubkey::from_str(
            "EQ7hw63aBS7aPQqXsoxaaBxiwbEzaAiY9Js6tCekkqxf",
        )
        .unwrap(),
        referrer_token_state: Pubkey::from_str("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD")
            .unwrap(),
        referrer_account: Pubkey::from_str("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD").unwrap(),
        sysvar_info: Pubkey::new_from_array(SysVarID.to_bytes()),
        token_program: TOKEN_PROGRAM_ID,
    };

    let mut hasher = Sha256::new();
    hasher.update(b"global:flashBorrowReserveLiquidity");
    let hash = hasher.finalize();
    let discriminator = &hash[..8];

    let mut data = Vec::with_capacity(16);
    data.extend_from_slice(discriminator);
    data.extend_from_slice(&liquidity.to_le_bytes());
    data.extend_from_slice(&borrow_index.to_le_bytes());

    Instruction {
        program_id: Pubkey::from_str("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD").unwrap(),
        accounts: vec![
            AccountMeta::new_readonly(accounts.user_transfer_authority, true),
            AccountMeta::new_readonly(accounts.lending_market_authority, false),
            AccountMeta::new_readonly(accounts.lending_market, false),
            AccountMeta::new(accounts.reserve, false),
            AccountMeta::new_readonly(accounts.reserve_liquidity_mint, false),
            AccountMeta::new(accounts.reserve_destination_liquidity, false),
            AccountMeta::new(accounts.user_source_liquidity, false),
            AccountMeta::new(accounts.reserve_liquidity_fee_receiver, false),
            AccountMeta::new(accounts.referrer_token_state, false),
            AccountMeta::new(accounts.referrer_account, false),
            AccountMeta::new_readonly(accounts.sysvar_info, false),
            AccountMeta::new_readonly(accounts.token_program, false),
        ],
        data: data,
    }
}

struct FlashBorrowAccounts {
    pub user_transfer_authority: Pubkey,
    pub lending_market_authority: Pubkey,
    pub lending_market: Pubkey,
    pub reserve: Pubkey,
    pub reserve_liquidity_mint: Pubkey,
    pub reserve_source_liquidity: Pubkey,
    pub user_destination_liquidity: spl_token::solana_program::pubkey::Pubkey,
    pub reserve_liquidity_fee_receiver: Pubkey,
    pub referrer_token_state: Pubkey,
    pub referrer_account: Pubkey,
    pub sysvar_info: Pubkey,
    pub token_program: Pubkey,
}

struct FlashRepayAccounts {
    pub user_transfer_authority: Pubkey,
    pub lending_market_authority: Pubkey,
    pub lending_market: Pubkey,
    pub reserve: Pubkey,
    pub reserve_liquidity_mint: Pubkey,
    pub reserve_destination_liquidity: Pubkey,
    pub user_source_liquidity: Pubkey,
    pub reserve_liquidity_fee_receiver: Pubkey,
    pub referrer_token_state: Pubkey,
    pub referrer_account: Pubkey,
    pub sysvar_info: Pubkey,
    pub token_program: Pubkey,
}
