use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum NoteError {
    #[error("Note title exceeds max length")]
    InvalidNoteTitleLength,
    #[error("Note body exceeds max length")]
    InvalidNoteBodyLength,
    #[error("Invalid update not authority")]
    InvalidNoteAuthority,
}

impl From<NoteError> for ProgramError {
    fn from(value: NoteError) -> Self {
        // custom program errors start at 1000
        ProgramError::Custom(value as u32)
    }
}
