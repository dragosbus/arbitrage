use sha2::{Digest, Sha256};
use solana_sdk::message::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::signer::Signer;
use solana_sdk::sysvar::ID as SysVarID;
use spl_token::solana_program::program_pack::Pack;
// use spl_token::ID as TOKEN_PROGRAM_ID;
use std::str::FromStr;
// use spl_token::solana_program::instruction::AccountMeta

// Official Kamino discriminators from their GitHub
const FLASH_BORROW_DISCRIMINATOR: [u8; 8] = [135, 231, 52, 167, 7, 52, 212, 193];
const FLASH_REPAY_DISCRIMINATOR: [u8; 8] = [185, 117, 0, 203, 96, 245, 180, 186];

// Helper function to debug instruction data
pub fn debug_instruction_data(instruction: &Instruction) {
    println!("Instruction program ID: {}", instruction.program_id);
    println!("Instruction data length: {}", instruction.data.len());
    println!("Instruction data: {:?}", instruction.data);

    println!("Accounts:");
    for (i, account_meta) in instruction.accounts.iter().enumerate() {
        println!("  Account #{}: {}", i, account_meta.pubkey);
        println!("    is_signer: {}", account_meta.is_signer);
        println!("    is_writable: {}", account_meta.is_writable);
    }
}

pub fn borrow_instruction_builder(
    payer: Pubkey,
    user_token_account: Pubkey,
    liquidity: u64,
) -> Instruction {
    let program_id = Pubkey::from_str("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD").unwrap();
    // Lending market account

    let accounts = FlashBorrowAccounts {
        user_transfer_authority: payer,
        lending_market_authority: Pubkey::from_str("9DrvZvyWh1HuAoZxvYWMvkf2XCzryCpGgHqrMjyDWpmo")
            .unwrap(),
        lending_market: Pubkey::from_str("7u3HeHxYDLhnCoErrtycNokbQYbWGzLs6JSDqGAv5PfF").unwrap(),
        reserve: Pubkey::from_str("d4A2prbA2whesmvHaL88BH6Ewn5N4bTSU2Ze8P6Bc4Q").unwrap(),
        reserve_liquidity_mint: Pubkey::from_str("So11111111111111111111111111111111111111112")
            .unwrap(),
        reserve_source_liquidity: Pubkey::from_str("GafNuUXj9rxGLn4y79dPu6MHSuPWeJR6UtTWuexpGh3U")
            .unwrap(),
        user_destination_liquidity: user_token_account,
        reserve_liquidity_fee_receiver: Pubkey::from_str(
            "3JNof8s453bwG5UqiXBLJc77NRQXezYYEBbk3fqnoKph",
        )
        .unwrap(),
        referrer_token_state: Pubkey::from_str("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD")
            .unwrap(),
        referrer_account: Pubkey::from_str("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD").unwrap(),
        sysvar_info: Pubkey::from_str("Sysvar1nstructions1111111111111111111111111").unwrap(),
        token_program: Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
    };

    let mut data = Vec::with_capacity(16);
    data.extend_from_slice(&FLASH_BORROW_DISCRIMINATOR);
    data.extend_from_slice(&liquidity.to_le_bytes());

    Instruction {
        program_id: Pubkey::from_str("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD").unwrap(),
        accounts: vec![
            AccountMeta::new(accounts.user_transfer_authority, true),
            AccountMeta::new_readonly(accounts.lending_market_authority, false),
            AccountMeta::new_readonly(accounts.lending_market, false),
            AccountMeta::new(accounts.reserve, false),
            AccountMeta::new_readonly(accounts.reserve_liquidity_mint, false),
            AccountMeta::new(accounts.reserve_source_liquidity, false),
            AccountMeta::new(accounts.user_destination_liquidity, false),
            AccountMeta::new(accounts.reserve_liquidity_fee_receiver, false),
            AccountMeta::new_readonly(accounts.referrer_token_state, false),
            AccountMeta::new_readonly(accounts.referrer_account, false),
            AccountMeta::new_readonly(
                Pubkey::from_str("Sysvar1nstructions1111111111111111111111111").unwrap(),
                false,
            ),
            AccountMeta::new_readonly(accounts.token_program, false),
        ],
        data: data,
    }
}

pub fn repay_instruction_builder(
    payer: solana_sdk::pubkey::Pubkey,
    liquidity: u64,
    user_token_account: solana_sdk::pubkey::Pubkey,
    borrow_index: u8,
) -> Instruction {
    let accounts = FlashRepayAccounts {
        user_transfer_authority: Pubkey::new_from_array(payer.to_bytes()),
        lending_market_authority: Pubkey::from_str("9DrvZvyWh1HuAoZxvYWMvkf2XCzryCpGgHqrMjyDWpmo")
            .unwrap(),
        lending_market: Pubkey::from_str("7u3HeHxYDLhnCoErrtycNokbQYbWGzLs6JSDqGAv5PfF").unwrap(),
        reserve: Pubkey::from_str("d4A2prbA2whesmvHaL88BH6Ewn5N4bTSU2Ze8P6Bc4Q").unwrap(),
        reserve_liquidity_mint: Pubkey::from_str("So11111111111111111111111111111111111111112")
            .unwrap(),
        reserve_destination_liquidity: Pubkey::from_str(
            "GafNuUXj9rxGLn4y79dPu6MHSuPWeJR6UtTWuexpGh3U",
        )
        .unwrap(),
        user_source_liquidity: user_token_account,
        reserve_liquidity_fee_receiver: Pubkey::from_str(
            "3JNof8s453bwG5UqiXBLJc77NRQXezYYEBbk3fqnoKph",
        )
        .unwrap(),
        referrer_token_state: Pubkey::from_str("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD")
            .unwrap(),
        referrer_account: Pubkey::from_str("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD").unwrap(),
        sysvar_info: Pubkey::from_str("Sysvar1nstructions1111111111111111111111111").unwrap(),
        token_program: Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
    };

    let mut data = Vec::with_capacity(17);
    data.extend_from_slice(&FLASH_REPAY_DISCRIMINATOR);
    data.extend_from_slice(&liquidity.to_le_bytes());
    data.push(borrow_index);

    Instruction {
        program_id: Pubkey::from_str("KLend2g3cP87fffoy8q1mQqGKjrxjC8boSyAYavgmjD").unwrap(),
        accounts: vec![
            AccountMeta::new(accounts.user_transfer_authority, true),
            AccountMeta::new_readonly(accounts.lending_market_authority, false),
            AccountMeta::new_readonly(accounts.lending_market, false),
            AccountMeta::new(accounts.reserve, false),
            AccountMeta::new_readonly(accounts.reserve_liquidity_mint, false),
            AccountMeta::new(accounts.reserve_destination_liquidity, false),
            AccountMeta::new(accounts.user_source_liquidity, false),
            AccountMeta::new(accounts.reserve_liquidity_fee_receiver, false),
            AccountMeta::new_readonly(accounts.referrer_token_state, false),
            AccountMeta::new_readonly(accounts.referrer_account, false),
            AccountMeta::new_readonly(
                Pubkey::from_str("Sysvar1nstructions1111111111111111111111111").unwrap(),
                false,
            ),
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

use solana_client::rpc_client::RpcClient;
use spl_token::state::Account as TokenAccount;

pub fn diagnose_wsol_ata(
    rpc_client: &RpcClient,
    wallet_pubkey: &Pubkey,
    ata_address: &Pubkey,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("=== WSOL ATA DIAGNOSTIC ===");
    println!("Wallet: {}", wallet_pubkey);
    println!("ATA: {}", ata_address);

    // 1. Fetch account info
    let account_info = rpc_client.get_account(ata_address)?;
    println!("✓ Account exists");
    println!("  Owner: {}", account_info.owner);
    println!("  Lamports: {}", account_info.lamports);
    println!("  Data length: {}", account_info.data.len());

    // 2. Verify Token Program ownership
    if account_info.owner != Pubkey::new_from_array(spl_token::ID.to_bytes()) {
        println!(
            "✗ WRONG OWNER - Expected: {}, Got: {}",
            spl_token::ID,
            account_info.owner
        );
        return Err("Wrong owner".into());
    }
    println!("✓ Owned by Token Program");

    // 3. Deserialize token account data
    let token_account = TokenAccount::unpack(&account_info.data)?;
    println!("✓ Successfully deserialized as token account");

    // 4. Check critical fields
    println!("=== TOKEN ACCOUNT DETAILS ===");
    println!("Mint: {}", token_account.mint);
    println!("Owner: {}", token_account.owner);
    println!("Amount: {}", token_account.amount);
    println!("Delegate: {:?}", token_account.delegate);
    println!("State: {:?}", token_account.state);
    println!("Is Native: {:?}", token_account.is_native);
    println!("Delegated Amount: {}", token_account.delegated_amount);
    println!("Close Authority: {:?}", token_account.close_authority);

    // 5. CRITICAL CHECKS for flash loans
    let wsol_mint = Pubkey::from_str("So11111111111111111111111111111111111111112")?;

    // Check mint
    if token_account.mint
        != spl_token::solana_program::pubkey::Pubkey::new_from_array(wsol_mint.to_bytes())
    {
        println!(
            "✗ WRONG MINT - Expected WSOL: {}, Got: {}",
            wsol_mint, token_account.mint
        );
        return Err("Wrong mint".into());
    }
    println!("✓ Correct WSOL mint");

    // Check owner
    if token_account.owner
        != spl_token::solana_program::pubkey::Pubkey::new_from_array(wallet_pubkey.to_bytes())
    {
        println!(
            "✗ WRONG TOKEN ACCOUNT OWNER - Expected: {}, Got: {}",
            wallet_pubkey, token_account.owner
        );
        return Err("Wrong token account owner".into());
    }
    println!("✓ Correct owner");

    // Check state
    if token_account.state != spl_token::state::AccountState::Initialized {
        println!(
            "✗ ACCOUNT NOT INITIALIZED - State: {:?}",
            token_account.state
        );
        return Err("Account not initialized".into());
    }
    println!("✓ Account initialized");

    // Check if it's native (this is crucial for WSOL)
    match token_account.is_native {
        spl_token::solana_program::program_option::COption::Some(lamports) => {
            println!("✓ Native WSOL account with {} lamports", lamports);
            if lamports == 0 {
                println!("⚠ WARNING: Native account has 0 lamports - this might cause issues");
            }
        }
        spl_token::solana_program::program_option::COption::None => {
            println!("✗ NOT A NATIVE WSOL ACCOUNT - This is likely the problem!");
            println!("  This account exists but isn't properly wrapped SOL");
            println!("  You need to wrap some SOL first");
            return Err("Not a native WSOL account".into());
        }
    }

    // Check delegate
    if token_account.delegate.is_some() {
        println!(
            "⚠ WARNING: Account has delegate: {:?}",
            token_account.delegate
        );
        println!("  Delegated amount: {}", token_account.delegated_amount);
        println!("  This might interfere with flash loans");
    }

    // Check close authority
    if token_account.close_authority.is_some() {
        println!(
            "⚠ INFO: Account has close authority: {:?}",
            token_account.close_authority
        );
    }

    // 6. Check if account has sufficient balance for flash loan
    println!("=== BALANCE CHECK ===");
    println!(
        "Token balance: {} lamports ({} SOL)",
        token_account.amount,
        token_account.amount as f64 / 1_000_000_000.0
    );

    if token_account.amount == 0 {
        println!("⚠ WARNING: Account has zero balance");
        println!("  Flash loans might still work, but you need balance for fees");
    }

    // 7. Verify this is the correct ATA
    let expected_ata = spl_associated_token_account::get_associated_token_address(
        &spl_associated_token_account::solana_program::pubkey::Pubkey::new_from_array(
            wallet_pubkey.to_bytes(),
        ),
        &spl_associated_token_account::solana_program::pubkey::Pubkey::from_str(
            "So11111111111111111111111111111111111111112",
        )?,
    );
    let expected_ata_sdk = Pubkey::new_from_array(expected_ata.to_bytes());

    if *ata_address != expected_ata_sdk {
        println!("✗ THIS IS NOT YOUR STANDARD ATA!");
        println!("  Expected: {}", expected_ata_sdk);
        println!("  Using:    {}", ata_address);
        println!("  This could be the source of your problems!");
    } else {
        println!("✓ This is your correct standard ATA");
    }

    println!("=== DIAGNOSTIC COMPLETE ===");

    Ok(())
}

// Usage in your main function
pub fn run_diagnostic(rpc_client: &RpcClient, wallet: &Keypair) {
    // Your calculated ATA
    let uu = spl_associated_token_account::get_associated_token_address(
        &spl_associated_token_account::solana_program::pubkey::Pubkey::new_from_array(
            wallet.pubkey().to_bytes(),
        ),
        &spl_associated_token_account::solana_program::pubkey::Pubkey::from_str(
            "So11111111111111111111111111111111111111112",
        )
        .unwrap(),
    );
    let ata_pubkey = Pubkey::new_from_array(uu.to_bytes());

    match diagnose_wsol_ata(rpc_client, &wallet.pubkey(), &ata_pubkey) {
        Ok(_) => println!("✓ All WSOL ATA checks passed!"),
        Err(e) => {
            println!("✗ WSOL ATA diagnostic failed: {}", e);
            println!("\nTO FIX:");
            println!("1. Make sure your ATA is properly created");
            println!("2. Wrap some SOL using: spl-token wrap <amount>");
            println!("3. Or use system transfer + sync_native instructions");
        }
    }
}
