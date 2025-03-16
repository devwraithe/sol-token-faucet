pub use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct FaucetState {
    pub admin: Pubkey,
    pub amount: u64,
}
