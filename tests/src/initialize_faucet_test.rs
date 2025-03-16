use sol_token_faucet::instructions::FaucetInstruction;
use sol_token_faucet::state::{BorshDeserialize, FaucetState};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    sysvar,
};
use solana_program_test::*;
use solana_sdk::msg;
use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use solana_sdk::{hash::Hash, signature::Signer, system_instruction, transaction::Transaction};

use crate::test_utils::*;

pub struct InitializeFaucet {
    pub banks_client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhash: Hash,
    pub faucet_pubkey: Pubkey,
    pub admin_keypair: Keypair,
}

pub async fn initialize_faucet() -> InitializeFaucet {
    let test_ctx = setup_test_env().await;
    let TestContext {
        banks_client,
        payer,
        recent_blockhash,
        faucet_keypair,
        admin_keypair,
        rent_exempt_balance,
    } = test_ctx;

    // generate keys
    let admin_pubkey = admin_keypair.pubkey();
    let faucet_pubkey = faucet_keypair.pubkey();

    // fund admin account
    let fund_admin_ix =
        system_instruction::transfer(&payer.pubkey(), &admin_pubkey, 10_000_000_000);

    let fund_admin_tx = Transaction::new_signed_with_payer(
        &[fund_admin_ix],
        Some(&payer.pubkey()),
        &[&payer],
        recent_blockhash,
    );

    banks_client
        .process_transaction(fund_admin_tx)
        .await
        .unwrap();

    msg!("admin account funded");

    // initialize faucet
    let init_amount = 1_000_000_000; // 1 sol
    let instruction_data = FaucetInstruction::InitializeFaucet {
        amount: init_amount,
    };

    let instruction = Instruction::new_with_borsh(
        PROGRAM_ID,
        &instruction_data,
        vec![
            AccountMeta::new(faucet_pubkey, false),
            AccountMeta::new_readonly(admin_pubkey, true),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
    );
    let mut transaction = Transaction::new_with_payer(&[instruction], Some(&payer.pubkey()));
    transaction.sign(&[&payer, &admin_keypair], recent_blockhash);
    banks_client.process_transaction(transaction).await.unwrap();

    // fund faucet account
    let fund_faucet_ix = system_instruction::transfer(&admin_pubkey, &faucet_pubkey, init_amount);

    let fund_faucet_tx = Transaction::new_signed_with_payer(
        &[fund_faucet_ix],
        Some(&payer.pubkey()),
        &[&payer, &admin_keypair],
        recent_blockhash,
    );

    msg!("before initialized");

    banks_client
        .process_transaction(fund_faucet_tx)
        .await
        .unwrap();

    msg!("after initialized");

    // verify faucet state
    let faucet_account = banks_client
        .get_account(faucet_pubkey)
        .await
        .unwrap()
        .expect("faucet account not found");

    let faucet_state = FaucetState::try_from_slice(&faucet_account.data).unwrap();
    assert_eq!(faucet_state.admin, admin_pubkey);
    assert_eq!(faucet_state.amount, init_amount);
    msg!("Faucet account data length: {}", faucet_account.data.len());

    // verify faucet sol
    let faucet_balance = banks_client.get_balance(faucet_pubkey).await.unwrap();
    println!(
        "Faucet balance: {}, Expected: {}",
        faucet_balance,
        rent_exempt_balance + init_amount
    );
    assert!(
        faucet_balance >= rent_exempt_balance + init_amount,
        "faucet was not properly funded!"
    );

    // return
    InitializeFaucet {
        banks_client,
        payer,
        recent_blockhash,
        faucet_pubkey,
        admin_keypair,
    }
}
