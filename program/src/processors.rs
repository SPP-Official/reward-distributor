use {
    crate::{
        error::CustomError,
        instruction::RewardArgs,
        pda::{AccountDeserialize, Discriminator, VaultAccount},
    },
    solana_program::{
        account_info::{next_account_info, AccountInfo},
        entrypoint::ProgramResult,
        msg,
        program::{invoke, invoke_signed},
        program_error::ProgramError,
        program_pack::Pack,
        pubkey::Pubkey,
        system_instruction,
        sysvar::{rent::Rent, Sysvar},
    },
};

pub fn process_init<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let authority = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let vault_pda = next_account_info(accounts_iter)?;
    let mint_ata = next_account_info(accounts_iter)?;
    let system_program = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let rent_sysvar = next_account_info(accounts_iter)?;
    let associated_token_program = next_account_info(accounts_iter)?;

    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let (pda, bump_seed) = Pubkey::find_program_address(&[b"vault"], program_id);

    if *vault_pda.key != pda {
        return Err(CustomError::IncorrectPdaKey.into());
    }

    // Make sure the vault can only be initialized once
    if !vault_pda.data_is_empty() {
        return Err(CustomError::AlreadyInitialized.into());
    }

    let rent = Rent::get()?;
    let space = 8 + VaultAccount::size();
    let lamports = rent.minimum_balance(space);

    // Create vault PDA
    invoke_signed(
        &system_instruction::create_account(
            authority.key,
            &pda,
            lamports,
            space as u64,
            program_id,
        ),
        &[authority.clone(), vault_pda.clone(), system_program.clone()],
        &[&[b"vault", &[bump_seed]]],
    )?;

    let vault_account = VaultAccount {
        mint: *mint.key,
        authority: *authority.key,
    };

    {
        let mut data = vault_pda.try_borrow_mut_data()?;
        data[0] = VaultAccount::discriminator();

        let account_bytes: &[u8] = bytemuck::bytes_of(&vault_account);
        data[8..8 + account_bytes.len()].copy_from_slice(account_bytes);
    }

    msg!("Created vault. Vault = {:?}", vault_account);

    invoke(
        &spl_associated_token_account::instruction::create_associated_token_account(
            authority.key,
            vault_pda.key,
            mint.key,
            token_program.key,
        ),
        &[
            authority.as_ref().clone(),
            mint_ata.as_ref().clone(),
            vault_pda.as_ref().clone(),
            mint.as_ref().clone(),
            system_program.as_ref().clone(),
            token_program.as_ref().clone(),
            rent_sysvar.as_ref().clone(),
            associated_token_program.as_ref().clone(),
        ],
    )?;

    Ok(())
}

pub fn process_reward<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    args: RewardArgs,
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();

    let authority = next_account_info(accounts_iter)?;
    let mint = next_account_info(accounts_iter)?;
    let vault_pda = next_account_info(accounts_iter)?;
    let mint_ata = next_account_info(accounts_iter)?;
    let destination_ata = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    if !authority.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    if *mint_ata.owner != spl_token::id() {
        return Err(CustomError::InvalidTokenAccount.into());
    }

    if *destination_ata.owner != spl_token::id() {
        return Err(CustomError::InvalidTokenAccount.into());
    }

    let (pda, bump_seed) = Pubkey::find_program_address(&[b"vault"], program_id);

    if *vault_pda.key != pda {
        return Err(CustomError::IncorrectPdaKey.into());
    }

    // Make sure the vault is already initialized
    if vault_pda.data_is_empty() {
        return Err(CustomError::NotInitialized.into());
    }

    let (vault_mint, vault_authority) = {
        let data = vault_pda.try_borrow_data().unwrap();
        let vault_account = VaultAccount::try_from_bytes(&data)?;
        (vault_account.mint, vault_account.authority)
    };

    if !mint.key.eq(&vault_mint) {
        return Err(CustomError::DoesNotSupportMint.into());
    }

    if !authority.key.eq(&vault_authority) {
        return Err(ProgramError::IllegalOwner);
    }

    if args.amount == 0 {
        return Err(CustomError::InvalidAmount.into());
    }

    let balance = spl_token::state::Account::unpack(&mint_ata.try_borrow_data()?)?;

    if balance.amount < args.amount {
        return Err(CustomError::InsufficientFunds.into());
    }

    invoke_signed(
        &spl_token::instruction::transfer(
            &token_program.key,
            &mint_ata.key,
            &destination_ata.key,
            &pda,
            &[],
            args.amount,
        )?,
        &[
            token_program.as_ref().clone(),
            mint_ata.as_ref().clone(),
            destination_ata.as_ref().clone(),
            vault_pda.as_ref().clone(),
        ],
        &[&[b"vault", &[bump_seed]]],
    )?;

    Ok(())
}
