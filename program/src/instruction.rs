use borsh::BorshDeserialize;
use solana_program::{program_error::ProgramError, pubkey::Pubkey};

pub enum NoteInstruction {
    CreateNote {
        title: String,
        body: String,
        authority: Pubkey,
    },
    UpdateNote {
        title: String,
        body: String,
    },
    DeleteNote,
}

#[derive(BorshDeserialize)]
struct NoteInstructionPayload {
    title: String,
    body: String,
    authority: Pubkey,
}

impl NoteInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (&variant, rest) = input
            .split_first()
            .ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match variant {
            0 => {
                let payload = NoteInstructionPayload::try_from_slice(rest).unwrap();
                Self::CreateNote {
                    title: payload.title,
                    body: payload.body,
                    authority: payload.authority,
                }
            }
            1 => {
                let payload = NoteInstructionPayload::try_from_slice(rest).unwrap();
                Self::UpdateNote {
                    title: payload.title,
                    body: payload.body,
                }
            }
            2 => Self::DeleteNote,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }
}
