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
