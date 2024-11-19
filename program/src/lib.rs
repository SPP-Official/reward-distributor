pub mod error;
pub mod instruction;
pub mod pda;
pub mod processors;

use {
    instruction::ProgramInstruction,
    processors::{process_init, process_reward},
    solana_program::{
        self, account_info::AccountInfo, declare_id, entrypoint::ProgramResult, msg,
        program_error::ProgramError, pubkey::Pubkey,
    },
};

pub const PROGRAM_ID: Pubkey =
    solana_program::pubkey!("PLAYcZHpkkcLiWY2Csw6bcUbbHh85T3tCqnwsA4qBwh");

declare_id!(PROGRAM_ID);

#[cfg(not(feature = "no-entrypoint"))]
solana_program::entrypoint!(process_instruction);

pub fn process_instruction<'info>(
    program_id: &Pubkey,
    accounts: &'info [AccountInfo<'info>],
    data: &[u8],
) -> ProgramResult {
    if program_id.ne(&crate::id()) {
        return Err(ProgramError::IncorrectProgramId);
    }

    let instruction = ProgramInstruction::unpack(data)?;

    match instruction {
        ProgramInstruction::Init => {
            msg!("Init");
            process_init(program_id, accounts)?
        }
        ProgramInstruction::Reward(args) => {
            msg!("Reward");
            process_reward(program_id, accounts, args)?
        }
    }

    Ok(())
}
