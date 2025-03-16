use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum FaucetInstruction {
    InitializeFaucet { amount: u64 }, // instruction variant
    RequestTokens,
}

// payload to represent instruction data
#[derive(BorshSerialize, BorshDeserialize)]
struct InitFaucetPayload {
    amount: u64,
}
#[derive(BorshSerialize, BorshDeserialize)]
struct RequestTokens {
    amount: u64,
}

impl FaucetInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => {
                let payload = InitFaucetPayload::try_from_slice(rest).unwrap();
                Self::InitializeFaucet {
                    amount: payload.amount,
                }
            }
            1 => Self::RequestTokens,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
