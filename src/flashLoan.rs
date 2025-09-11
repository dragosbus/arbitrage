use sha2::{Digest, Sha256};
use solana_sdk::message::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::sysvar::ID as SysVarID;
use spl_token::ID as TOKEN_PROGRAM_ID;
use std::str::FromStr;

pub fn borrow_instruction_builder(
    payer: Pubkey,
    user_token_account: Pubkey,
    liquidity: u64,
) -> Instruction {
    let accounts = FlashBorrowAccounts {
        user_transfer_authority: payer,
        lending_market_authority: Pubkey::from_str("9DrvZvyWh1HuAoZxvYWMvkf2XCzryCpGgHqrMjyDWpmo")
            .unwrap(),
        lending_market: Pubkey::from_str("7u3HeHxYDLhnCoErrtycNokbQYbWGzLs6JSDqGAv5PfF").unwrap(),
        reserve: Pubkey::from_str("D6q6wuQSrifJKZYpR1M8R4YawnLDtDsMmWM1NbBmgJ59").unwrap(),
        reserve_liquidity_mint: Pubkey::from_str("So11111111111111111111111111111111111111112")
            .unwrap(),
        reserve_source_liquidity: Pubkey::from_str("Bgq7trRgVMeq33yt235zM2onQ4bRDBsY5EWiTetF4qw6")
            .unwrap(),
        user_destination_liquidity: user_token_account,
        reserve_liquidity_fee_receiver: Pubkey::from_str(
            "BbDUrk1bVtSixgQsPLBJFZEF7mwGstnD5joA1WzYvYFX",
        )
        .unwrap(),
        referrer_token_state: Pubkey::from_str("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD")
            .unwrap(),
        referrer_account: Pubkey::from_str("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD").unwrap(),
        sysvar_info: Pubkey::from_str("Sysvar1nstructions1111111111111111111111111").unwrap(),
        token_program: Pubkey::new_from_array(TOKEN_PROGRAM_ID.to_bytes()),
    };

    let mut hasher = Sha256::new();
    hasher.update(b"global:flash_borrow_reserve_liquidity");
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
        lending_market_authority: Pubkey::from_str("9DrvZvyWh1HuAoZxvYWMvkf2XCzryCpGgHqrMjyDWpmo")
            .unwrap(),
        lending_market: Pubkey::from_str("7u3HeHxYDLhnCoErrtycNokbQYbWGzLs6JSDqGAv5PfF").unwrap(),
        reserve: Pubkey::from_str("D6q6wuQSrifJKZYpR1M8R4YawnLDtDsMmWM1NbBmgJ59").unwrap(),
        reserve_liquidity_mint: Pubkey::from_str("So11111111111111111111111111111111111111112")
            .unwrap(),
        reserve_destination_liquidity: Pubkey::from_str(
            "Bgq7trRgVMeq33yt235zM2onQ4bRDBsY5EWiTetF4qw6",
        )
        .unwrap(),
        user_source_liquidity: user_token_account,
        reserve_liquidity_fee_receiver: Pubkey::from_str(
            "BbDUrk1bVtSixgQsPLBJFZEF7mwGstnD5joA1WzYvYFX",
        )
        .unwrap(),
        referrer_token_state: Pubkey::from_str("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD")
            .unwrap(),
        referrer_account: Pubkey::from_str("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD").unwrap(),
        sysvar_info: Pubkey::from_str("Sysvar1nstructions1111111111111111111111111").unwrap(),
        token_program: Pubkey::new_from_array(TOKEN_PROGRAM_ID.to_bytes()),
    };

    let mut hasher = Sha256::new();
    hasher.update(b"global:flash_repay_reserve_liquidity");
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
    pub user_destination_liquidity: Pubkey,
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
