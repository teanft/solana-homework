use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct NoteAccountState {
    pub authority: Pubkey,
    pub title: String,
    pub body: String,
}

impl NoteAccountState {
    pub const MAX_TITLE_SIZE: usize = 10;
    pub const MAX_BODY_SIZE: usize = 100;
    pub const MAX_SIZE: usize = 32 + Self::MAX_TITLE_SIZE + Self::MAX_BODY_SIZE;
}
