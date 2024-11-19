use {
    bytemuck::{Pod, Zeroable},
    solana_program::{program_error::ProgramError, pubkey::Pubkey},
};

pub trait Discriminator {
    fn discriminator() -> u8;
}

pub trait AccountDeserialize {
    fn try_from_bytes(data: &[u8]) -> Result<&Self, ProgramError>;
    fn try_from_bytes_mut(data: &mut [u8]) -> Result<&mut Self, ProgramError>;
}

#[macro_export]
macro_rules! impl_to_bytes {
    ($struct_name:ident) => {
        impl $struct_name {
            pub fn to_bytes(&self) -> &[u8] {
                bytemuck::bytes_of(self)
            }
        }
    };
}

#[macro_export]
macro_rules! impl_account_from_bytes {
    ($struct_name:ident) => {
        impl $crate::pda::AccountDeserialize for $struct_name {
            fn try_from_bytes(
                data: &[u8],
            ) -> Result<&Self, solana_program::program_error::ProgramError> {
                if Self::discriminator().ne(&data[0]) {
                    return Err(solana_program::program_error::ProgramError::InvalidAccountData);
                }
                bytemuck::try_from_bytes::<Self>(&data[8..]).or(Err(
                    solana_program::program_error::ProgramError::InvalidAccountData,
                ))
            }
            fn try_from_bytes_mut(
                data: &mut [u8],
            ) -> Result<&mut Self, solana_program::program_error::ProgramError> {
                if Self::discriminator().ne(&data[0]) {
                    return Err(solana_program::program_error::ProgramError::InvalidAccountData);
                }
                bytemuck::try_from_bytes_mut::<Self>(&mut data[8..]).or(Err(
                    solana_program::program_error::ProgramError::InvalidAccountData,
                ))
            }
        }
    };
}

#[derive(Clone, Copy, Pod, Zeroable, Debug, Default, PartialEq, Eq)]
#[repr(C)]
pub struct VaultAccount {
    pub mint: Pubkey,
    pub authority: Pubkey,
}

impl_to_bytes!(VaultAccount);
impl_account_from_bytes!(VaultAccount);

impl Discriminator for VaultAccount {
    fn discriminator() -> u8 {
        0
    }
}

impl VaultAccount {
    pub fn size() -> usize {
        std::mem::size_of::<VaultAccount>()
    }
}
