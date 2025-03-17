use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

use crate::instructions::FaucetInstruction;
use initialize_faucet::initialize_faucet;
use refill_faucet::refill_faucet;
use request_tokens::request_tokens;

mod initialize_faucet;
mod refill_faucet;
mod request_tokens;

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = FaucetInstruction::unpack(instruction_data)?;

        match instruction {
            FaucetInstruction::InitializeFaucet { amount } => {
                initialize_faucet(program_id, accounts, amount)
                    .expect("failed to initialize faucet");
            }
            FaucetInstruction::RequestTokens => {
                request_tokens(program_id, accounts).expect("failed to request tokens");
            }
            FaucetInstruction::RefillFaucet { amount } => {
                refill_faucet(program_id, accounts, amount).expect("failed to refill faucet");
            }
        }

        Ok(())
    }
}
