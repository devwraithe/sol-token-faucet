use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, msg, pubkey::Pubkey};

use crate::instructions::FaucetInstruction;
use initialize_faucet::initialize_faucet;

mod initialize_faucet;

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
        }

        Ok(())
    }
}
