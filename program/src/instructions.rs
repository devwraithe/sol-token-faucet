use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::program_error::ProgramError;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum FaucetInstruction {
    InitializeFaucet { amount: u64 }, // instruction variant
    RequestTokens,
    RefillFaucet { amount: u64 },
}

// payload to represent instruction data
#[derive(BorshSerialize, BorshDeserialize)]
struct InitFaucetPayload {
    amount: u64,
}
#[derive(BorshSerialize, BorshDeserialize)]
struct RefillFaucet {
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
            2 => {
                let payload = RefillFaucet::try_from_slice(rest).unwrap();
                Self::RefillFaucet {
                    amount: payload.amount,
                }
            }
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
