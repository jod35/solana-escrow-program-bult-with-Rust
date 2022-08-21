use thiserror::Error;
use solana_program::program_error::ProgramError;
use crate::{instruction::EscrowInstruction};



#[derive(Error,Copy,Clone,Debug)]
pub enum EscrowError {
    ///Invalid Instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    #[error("No Rent Exempted")]
    NotRentExempt
}


impl From<EscrowError> for ProgramError{
    fn from(e:EscrowError) -> Self {
        ProgramError::Custom(e as u32)
    }
}


