use {
    num_derive::FromPrimitive,
    solana_program::{
        self,
        decode_error::DecodeError,
        msg,
        program_error::{PrintProgramError, ProgramError},
    },
    thiserror::Error,
};

#[repr(u32)]
#[derive(Clone, Debug, Eq, Error, FromPrimitive, PartialEq)]
pub enum CustomError {
    #[error("IncorrectPdaKey")]
    IncorrectPdaKey,
    #[error("AlreadyInitialized")]
    AlreadyInitialized,
    #[error("NotInitialized")]
    NotInitialized,
    #[error("DoesNotSupportMint")]
    DoesNotSupportMint,
    #[error("InvalidTokenAccount")]
    InvalidTokenAccount,
    #[error("InvalidAmount")]
    InvalidAmount,
    #[error("InsufficientFunds")]
    InsufficientFunds,
}

impl From<CustomError> for ProgramError {
    fn from(e: CustomError) -> Self {
        ProgramError::Custom(e as u32)
    }
}

impl PrintProgramError for CustomError {
    fn print<E>(&self)
    where
        E: 'static
            + std::error::Error
            + DecodeError<E>
            + PrintProgramError
            + num_traits::FromPrimitive,
    {
        match self {
            CustomError::IncorrectPdaKey => msg!("CustomError: IncorrectPdaKey"),
            CustomError::AlreadyInitialized => msg!("CustomError: AlreadyInitialized"),
            CustomError::NotInitialized => msg!("CustomError: NotInitialized"),
            CustomError::DoesNotSupportMint => msg!("CustomError: DoesNotSupportMint"),
            CustomError::InvalidTokenAccount => msg!("CustomError: InvalidTokenAccount"),
            CustomError::InvalidAmount => msg!("CustomError: InvalidAmount"),
            CustomError::InsufficientFunds => msg!("CustomError: InsufficientFunds"),
        }
    }
}
