use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::instructions::FaucetInstruction;
use initialize_faucet::initialize_faucet;
use request_tokens::request_tokens;

mod initialize_faucet;
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
                msg!("instruction: request tokens");
                request_tokens(program_id, accounts).expect("failed to request tokens");
            }
        }

        Ok(())
    }
}
