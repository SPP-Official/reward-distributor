use {
    borsh::{BorshDeserialize, BorshSerialize},
    solana_program::program_error::ProgramError,
};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum ProgramInstruction {
    Init,
    Reward(RewardArgs),
}

#[derive(BorshSerialize, BorshDeserialize, Debug, Clone, PartialEq)]
pub struct RewardArgs {
    pub amount: u64,
}

impl ProgramInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&tag, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        match tag {
            0 => Ok(ProgramInstruction::Init),
            1 => {
                let args = RewardArgs::try_from_slice(rest)
                    .map_err(|_| ProgramError::InvalidInstructionData)?;

                Ok(ProgramInstruction::Reward(args))
            }
            _ => Err(ProgramError::InvalidInstructionData),
        }
    }
}
