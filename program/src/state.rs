use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshDeserialize, BorshSerialize, Debug, Clone)]
pub struct ChatMessage {
    pub messages: Vec<String>,
    pub sender: String,
    pub reciever: String,
    pub created_on: i64,
}
