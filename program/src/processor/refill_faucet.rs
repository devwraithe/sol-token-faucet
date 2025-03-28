use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError::*,
    pubkey::Pubkey,
    system_instruction,
};

pub fn refill_faucet(program_id: &Pubkey, accounts: &[AccountInfo], amount: u64) -> ProgramResult {
    msg!("instruction: refill faucet");

    let accounts_iter = &mut accounts.iter();

    // required accounts
    let faucet_account = next_account_info(accounts_iter)?;
    let admin_account = next_account_info(accounts_iter)?;

    // verify admin as signer
    if !admin_account.is_signer {
        msg!("admin authority is not a signer");
        return Err(MissingRequiredSignature);
    }

    // verify faucet belongs to program
    if faucet_account.owner != program_id {
        msg!("faucet account is not owned by the program");
        return Err(IncorrectProgramId);
    }

    // perform the transfer from admin to faucet
    msg!(
        "transferring {} lamports from admin {} to faucet {}",
        amount,
        admin_account.key,
        faucet_account.key
    );

    invoke(
        &system_instruction::transfer(admin_account.key, faucet_account.key, amount),
        &[admin_account.clone(), faucet_account.clone()],
    )?;

    msg!("faucet successfully refilled!");

    Ok(())
}
