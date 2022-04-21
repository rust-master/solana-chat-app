use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug, Copy, Clone, PartialEq)]
pub enum ChatError {
    #[error("Instruction not implemented.")]
    NotImplemented,
}

impl From<ChatError> for ProgramError {
    fn from(e: ChatError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
