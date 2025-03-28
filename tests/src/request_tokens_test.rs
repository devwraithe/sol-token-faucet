use sol_token_faucet::instructions::FaucetInstruction;
use solana_program::system_program;
use solana_program_test::*;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    signature::{keypair::Keypair, Signer},
    transaction::Transaction,
};

use crate::{
    initialize_faucet_test::{initialize_faucet, InitializeFaucet},
    test_utils::PROGRAM_ID,
};

#[tokio::test]
async fn request_tokens() {
    let initialize_faucet = initialize_faucet(10_000_000_000, 1_000_000_000).await;
    let InitializeFaucet {
        mut banks_client,
        payer,
        recent_blockhash,
        faucet_pubkey,
        admin_keypair: _,
    } = initialize_faucet;

    let user_keypair = Keypair::new();
    let user_pubkey = user_keypair.pubkey();

    // instructions for request tokens
    let request_ix = Instruction::new_with_borsh(
        PROGRAM_ID,
        &FaucetInstruction::RequestTokens,
        vec![
            AccountMeta::new(faucet_pubkey, false),
            AccountMeta::new(user_pubkey, false),
            AccountMeta::new_readonly(system_program::id(), false),
        ],
    );
    let mut request_tx = Transaction::new_with_payer(&[request_ix], Some(&payer.pubkey()));
    request_tx.sign(&[&payer], recent_blockhash);
    banks_client.process_transaction(request_tx).await.unwrap();

    // verify user balance
    let user_balance = banks_client.get_balance(user_pubkey).await.unwrap();
    assert_eq!(user_balance, 1_000_000_000);
}
