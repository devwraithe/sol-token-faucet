use sol_token_faucet::instructions::FaucetInstruction;
use sol_token_faucet::process_instruction;
use sol_token_faucet::state::{BorshDeserialize, FaucetState};
use solana_program::{
    instruction::{AccountMeta, Instruction},
    sysvar,
};
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    pubkey::Pubkey,
    rent::Rent,
    signature::{Signer, keypair::Keypair},
    system_instruction,
    transaction::Transaction,
};
use std::mem::size_of; // for size calculation

#[tokio::test]
async fn initialize_faucet() {
    let program_id = Pubkey::new_unique();
    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "initialize_faucet",
        program_id,
        processor!(process_instruction),
    );

    // generate admin & faucet keypairs
    let admin_keypair = Keypair::new();
    let faucet_keypair = Keypair::new();
    let admin_pubkey = admin_keypair.pubkey();
    let faucet_pubkey = faucet_keypair.pubkey();

    // calculate faucet rent-exempt balance
    let space = size_of::<FaucetState>();
    let rent = Rent::default();
    let rent_exempt_balance = rent.minimum_balance(space);

    // add accounts to program test
    program_test.add_account(
        faucet_pubkey,
        Account {
            lamports: rent_exempt_balance,
            owner: program_id,    // program owned
            data: vec![0; space], // pre-allocate space
            executable: false,
            ..Account::default()
        },
    );
    // program_test.add_account(
    //     admin_pubkey,
    //     Account {
    //         lamports: 10_000_000_000, // 10 SOL
    //         owner: solana_program::system_program::id(),
    //         ..Account::default()
    //     },
    // );

    // start test environment
    let (banks_client, payer, recent_blockhash) = program_test.start().await;

    // create admin account
    let create_admin_ix = system_instruction::create_account(
        &payer.pubkey(),
        &admin_pubkey,
        rent.minimum_balance(0), // min bal for acct with no data
        0,
        &solana_program::system_program::id(),
    );

    let create_admin_tx = Transaction::new_signed_with_payer(
        &[create_admin_ix],
        Some(&payer.pubkey()),
        &[&payer, &admin_keypair],
        recent_blockhash,
    );

    banks_client
        .process_transaction(create_admin_tx)
        .await
        .unwrap();

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

    // initialize faucet
    let init_amount = 1_000_000_000;
    let instruction_data = FaucetInstruction::InitializeFaucet {
        amount: init_amount,
    };

    let instruction = Instruction::new_with_borsh(
        program_id,
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

    banks_client
        .process_transaction(fund_faucet_tx)
        .await
        .unwrap();

    // verify faucet state
    let faucet_account = banks_client
        .get_account(faucet_pubkey)
        .await
        .unwrap()
        .expect("faucet account not found");

    let faucet_state = FaucetState::try_from_slice(&faucet_account.data).unwrap();
    assert_eq!(faucet_state.admin, admin_pubkey);
    assert_eq!(faucet_state.amount, init_amount);

    // verify faucet sol
    let faucet_balance = banks_client.get_balance(faucet_pubkey).await.unwrap();
    assert!(
        faucet_balance >= rent_exempt_balance + init_amount,
        "faucet was not properly funded!"
    );
}
