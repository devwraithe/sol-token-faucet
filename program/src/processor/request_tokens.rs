use crate::state::FaucetState;
use borsh::BorshDeserialize;
use solana_program::{
    account_info::{AccountInfo, next_account_info},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError::*,
    pubkey::Pubkey,
};

pub fn request_tokens(program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    msg!("instruction: request tokens");

    let accounts_iter = &mut accounts.iter();

    // required accounts
    let faucet_account = next_account_info(accounts_iter)?;
    let user_account = next_account_info(accounts_iter)?;

    // verify faucet balance
    if **faucet_account.lamports.borrow() == 0 {
        msg!("faucet account is empty");
        return Err(InsufficientFunds);
    }

    // verify ownership
    if faucet_account.owner != program_id {
        msg!("faucet account is not owned by the program");
        return Err(IncorrectProgramId);
    }

    // load the faucet state
    let faucet_state = FaucetState::try_from_slice(&faucet_account.data.borrow())?;

    msg!("Expected program ID: {:?}", program_id);
    msg!("Actual owner of faucet account: {:?}", faucet_account.owner);

    // amount to transfer on each request
    let request_amount = faucet_state.amount;
    println!("faucet state amount: {:?}", faucet_state.amount);
    println!("request_amount: {:?}", request_amount);

    // transfer sol from faucet to user
    **faucet_account.try_borrow_mut_lamports()? -= request_amount;
    **user_account.try_borrow_mut_lamports()? += request_amount;

    // show response in console
    msg!(
        "tokens requested. transferred: {:?} to user: {:?}",
        request_amount,
        user_account.key
    );

    Ok(())
}
