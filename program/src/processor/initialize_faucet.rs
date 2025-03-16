use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError::*,
    pubkey::Pubkey,
    rent::Rent,
    sysvar::Sysvar,
};

use crate::state::FaucetState;

pub fn initialize_faucet(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    amount: u64,
) -> ProgramResult {
    msg!("instruction: initialize faucet");

    let accounts_iter = &mut accounts.iter();

    let faucet_account = next_account_info(accounts_iter)?;
    let admin_account = next_account_info(accounts_iter)?;
    let rent = &Rent::from_account_info(next_account_info(accounts_iter)?)?;

    // verify ownership
    if faucet_account.owner != program_id {
        msg!("faucet account is not owned by the program");
        return Err(InvalidAccountData);
    }

    // verify writability
    if !faucet_account.is_writable {
        msg!("faucet account is not writable");
        return Err(InvalidAccountData);
    }

    // verify signer
    if !admin_account.is_signer {
        msg!("admin authority is not a signer");
        return Err(MissingRequiredSignature);
    }

    // verify rent exemption
    if !rent.is_exempt(faucet_account.lamports(), faucet_account.data_len()) {
        msg!("faucet account is not rent exempt");
        return Err(AccountNotRentExempt);
    }

    // update the faucet state
    let faucet_state = FaucetState {
        admin: *admin_account.key,
        amount: amount, // amount to distribute
    };
    faucet_state.serialize(&mut &mut faucet_account.data.borrow_mut()[..])?;

    // log the response
    msg!("faucet initialized. {:?}", faucet_account.key);

    Ok(())
}
