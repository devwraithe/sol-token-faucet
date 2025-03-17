use solana_program_test::*;
use solana_sdk::{msg, signature::Signer, system_instruction, transaction::Transaction};

use crate::initialize_faucet_test::{InitializeFaucet, initialize_faucet};

#[tokio::test]
async fn refill_faucet() {
    let initialize_faucet = initialize_faucet(10_000_000_000, 100_000).await;
    let InitializeFaucet {
        banks_client,
        payer: _,
        recent_blockhash,
        faucet_pubkey,
        admin_keypair,
    } = initialize_faucet;

    // refill faucet account
    let refill_faucet_ix =
        system_instruction::transfer(&admin_keypair.pubkey(), &faucet_pubkey, 100_000_000);

    let refill_faucet_tx = Transaction::new_signed_with_payer(
        &[refill_faucet_ix],
        Some(&admin_keypair.pubkey()),
        &[&admin_keypair],
        recent_blockhash,
    );

    banks_client
        .process_transaction(refill_faucet_tx)
        .await
        .unwrap();

    msg!("faucet refilled");
}
