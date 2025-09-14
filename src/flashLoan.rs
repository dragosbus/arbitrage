use solana_sdk::message::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

// Official Kamino discriminators from their GitHub
const FLASH_BORROW_DISCRIMINATOR: [u8; 8] = [135, 231, 52, 167, 7, 52, 212, 193];
const FLASH_REPAY_DISCRIMINATOR: [u8; 8] = [185, 117, 0, 203, 96, 245, 180, 186];

pub fn flash_loan_instruction(payer: &Pubkey, user_ata: &Pubkey, data: Vec<u8>) -> Instruction {
    let program_id = Pubkey::from_str("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD").unwrap();
    let lending_market = Pubkey::from_str("7u3HeHxYDLhnCoErrtycNokbQYbWGzLs6JSDqGAv5PfF").unwrap();
    let lending_market_authority =
        Pubkey::from_str("9DrvZvyWh1HuAoZxvYWMvkf2XCzryCpGgHqrMjyDWpmo").unwrap();
    let reserve = Pubkey::from_str("d4A2prbA2whesmvHaL88BH6Ewn5N4bTSU2Ze8P6Bc4Q").unwrap();
    let reserve_liquidity_mint =
        Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap();
    let reserve_liquidity =
        Pubkey::from_str("GafNuUXj9rxGLn4y79dPu6MHSuPWeJR6UtTWuexpGh3U").unwrap();
    let reserve_liquidity_fee_receiver =
        Pubkey::from_str("3JNof8s453bwG5UqiXBLJc77NRQXezYYEBbk3fqnoKph").unwrap();
    let referrer_token_state =
        Pubkey::from_str("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD").unwrap();
    let referrer_account = Pubkey::from_str("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD").unwrap();
    let sysvar_info = Pubkey::from_str("Sysvar1nstructions1111111111111111111111111").unwrap();
    let token_program = Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap();

    Instruction {
        program_id,
        accounts: vec![
            AccountMeta::new(*payer, true),
            AccountMeta::new_readonly(lending_market_authority, false),
            AccountMeta::new_readonly(lending_market, false), //
            AccountMeta::new(reserve, false),
            AccountMeta::new_readonly(reserve_liquidity_mint, false),
            AccountMeta::new(reserve_liquidity, false), // source for borrow, destination for repay
            AccountMeta::new(*user_ata, false),
            AccountMeta::new(reserve_liquidity_fee_receiver, false),
            AccountMeta::new_readonly(referrer_token_state, false),
            AccountMeta::new_readonly(referrer_account, false),
            AccountMeta::new_readonly(sysvar_info, false),
            AccountMeta::new_readonly(token_program, false),
        ],
        data,
    }
}

pub fn borrow_instruction_builder(
    payer: Pubkey,
    user_token_account: Pubkey,
    liquidity: u64,
) -> Instruction {
    let mut data = Vec::with_capacity(16);
    data.extend_from_slice(&FLASH_BORROW_DISCRIMINATOR);
    data.extend_from_slice(&liquidity.to_le_bytes());

    flash_loan_instruction(&payer, &user_token_account, data)
}

pub fn repay_instruction_builder(
    payer: solana_sdk::pubkey::Pubkey,
    liquidity: u64,
    user_token_account: solana_sdk::pubkey::Pubkey,
    borrow_index: u8,
) -> Instruction {
    let mut data = Vec::with_capacity(17);
    data.extend_from_slice(&FLASH_REPAY_DISCRIMINATOR);
    data.extend_from_slice(&liquidity.to_le_bytes());
    data.push(borrow_index);

    flash_loan_instruction(&payer, &user_token_account, data)
}
