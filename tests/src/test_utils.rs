use sol_token_faucet::process_instruction;
use sol_token_faucet::state::FaucetState;
use solana_program_test::*;
use solana_sdk::{
    account::Account,
    hash::Hash,
    msg,
    pubkey::Pubkey,
    rent::Rent,
    signature::{keypair::Keypair, Signer},
    system_instruction,
    transaction::Transaction,
};
use std::mem::size_of; // for size calculation

pub const PROGRAM_ID: Pubkey = Pubkey::new_from_array([1; 32]);

pub struct TestContext {
    pub banks_client: BanksClient,
    pub payer: Keypair,
    pub recent_blockhash: Hash,
    pub faucet_keypair: Keypair,
    pub admin_keypair: Keypair,
    pub rent_exempt_balance: u64,
}

pub async fn setup_test_env() -> TestContext {
    let mut program_test = ProgramTest::default();
    program_test.add_program(
        "sol_token_faucet",
        PROGRAM_ID,
        processor!(process_instruction),
    );

    // create keypairs
    let admin_keypair = Keypair::new();
    let faucet_keypair = Keypair::new();

    let admin_pubkey = admin_keypair.pubkey();
    let faucet_pubkey = faucet_keypair.pubkey();

    // calculate faucet rent-exempt balance
    let space = size_of::<FaucetState>();
    let rent = Rent::default();
    let rent_exempt_balance: u64 = rent.minimum_balance(space);

    // add accounts to program test
    program_test.add_account(
        faucet_pubkey,
        Account {
            lamports: rent_exempt_balance,
            owner: PROGRAM_ID,    // program owned
            data: vec![0; space], // pre-allocate space
            executable: false,
            ..Account::default()
        },
    );

    // start test environment
    let (mut banks_client, payer, recent_blockhash) = program_test.start().await;

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

    msg!("admin account created");

    // return text context
    TestContext {
        banks_client,
        payer,
        recent_blockhash,
        faucet_keypair,
        admin_keypair,
        rent_exempt_balance,
    }
}
