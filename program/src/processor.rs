use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

use crate::instruction::ChatInstruction;
use crate::state::ChatMessage;

pub struct Processor {}

impl Processor {
    pub fn process_instruction(
        _program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = ChatInstruction::try_from_slice(instruction_data)
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        let account_iter = &mut accounts.iter();

        match instruction {
            ChatInstruction::SendMessage { message } => {
                msg!("Send Message Instruction Run");

                let send_ai = next_account_info(account_iter)?;
                let mut chat = ChatMessage::try_from_slice(&send_ai.data.borrow())?;

                chat.messages.push(message);

                chat.serialize(&mut *send_ai.data.borrow_mut())?;
            }
            ChatInstruction::GetMessages => {}
        }
        Ok(())
    }
}
