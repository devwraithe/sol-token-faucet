use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    signature::Keypair,
    signer::Signer,
    system_instruction, sysvar,
    transaction::Transaction,
};
use std::error::*;
use utils::{ensure_admin_keypair, save_faucet_pubkey};

mod utils;

const RPC_URL: &str = "https://api.devnet.solana.com";

fn initialize_faucet(
    admin_keypair: &Keypair,
    faucet_pubkey: &Pubkey,
    amount: u64,
) -> Result<(), Box<dyn Error>> {
    let client = RpcClient::new_with_commitment(RPC_URL, CommitmentConfig::confirmed());

    // create init faucet instruction
    let init_ix = Instruction {
        program_id: *faucet_pubkey,

        accounts: vec![
            AccountMeta::new(*faucet_pubkey, false),
            AccountMeta::new_readonly(admin_keypair.pubkey(), true),
            AccountMeta::new_readonly(sysvar::rent::id(), false),
        ],
        data: [].to_vec(),
    };

    // Create system transfer instruction to fund the faucet
    let transfer_ix = system_instruction::transfer(
        &admin_keypair.pubkey(),
        faucet_pubkey,
        amount, // Transfer `amount` lamports to the faucet
    );

    // create a txn
    let mut txn =
        Transaction::new_with_payer(&[init_ix, transfer_ix], Some(&admin_keypair.pubkey()));
    let recent_blockhash = client.get_latest_blockhash()?;
    txn.sign(&[admin_keypair], recent_blockhash);

    // send txn
    let signature = client.send_and_confirm_transaction(&txn)?;
    println!("faucet init with signature: {:?}", signature);

    // Save faucet pubkey to config
    save_faucet_pubkey(&faucet_pubkey);

    Ok(())
}

fn main() {
    let client = RpcClient::new_with_commitment(RPC_URL, CommitmentConfig::confirmed());

    let admin_keypair = ensure_admin_keypair();
    println!("Admin public key: {}", admin_keypair.pubkey());
    let admin_balance = client.get_balance(&admin_keypair.pubkey()).unwrap();
    println!("Admin balance: {}", admin_balance);

    // 🚀 Send transaction to initialize faucet (mocking a generated pubkey)
    let faucet_pubkey = Pubkey::new_unique(); // Replace with real faucet pubkey from transaction

    let amount = 1000000; // Amount to fund faucet (lamports)

    if let Err(e) = initialize_faucet(&admin_keypair, &faucet_pubkey, amount) {
        eprintln!("Failed to initialize faucet: {}", e);
    }
}
