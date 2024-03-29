use borsh::BorshSerialize;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    borsh1::try_from_slice_unchecked,
    entrypoint::ProgramResult,
    program::invoke,
    pubkey::Pubkey,
    rent::Rent,
    system_instruction,
    sysvar::Sysvar,
};

use crate::{error::NoteError, instruction::NoteInstruction, state::NoteAccountState};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = NoteInstruction::unpack(instruction_data)?;
    match instruction {
        NoteInstruction::CreateNote {
            title,
            body,
            authority,
        } => create_note(program_id, accounts, title, body, authority),
        NoteInstruction::UpdateNote { title, body } => {
            update_node(program_id, accounts, title, body)
        }
        NoteInstruction::DeleteNote => delete_note(program_id, accounts),
    }
}

fn create_note(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    body: String,
    authority: Pubkey,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let note_account = next_account_info(account_info_iter)?;
    let note = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    if title.len() > NoteAccountState::MAX_TITLE_SIZE {
        return Err(NoteError::InvalidNoteTitleLength.into());
    }

    if body.len() > NoteAccountState::MAX_BODY_SIZE {
        return Err(NoteError::InvalidNoteBodyLength.into());
    }

    let rent = Rent::get()?;
    let rent_lamports = rent.minimum_balance(NoteAccountState::MAX_SIZE);

    invoke(
        &system_instruction::create_account(
            note_account.key,
            note.key,
            rent_lamports,
            NoteAccountState::MAX_SIZE.try_into().unwrap(),
            program_id,
        ),
        &[note_account.clone(), note.clone(), system_program.clone()],
    )?;

    let mut note_data = try_from_slice_unchecked::<NoteAccountState>(&note.data.borrow()).unwrap();
    note_data.authority = authority;
    note_data.title = title;
    note_data.body = body;

    note_data.serialize(&mut &mut note.data.borrow_mut()[..])?;

    Ok(())
}

pub fn update_node(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    title: String,
    body: String,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let note_account = next_account_info(account_info_iter)?;
    let note = next_account_info(account_info_iter)?;

    if title.len() > NoteAccountState::MAX_TITLE_SIZE {
        return Err(NoteError::InvalidNoteTitleLength.into());
    }

    if body.len() > NoteAccountState::MAX_BODY_SIZE {
        return Err(NoteError::InvalidNoteBodyLength.into());
    }

    let mut note_data = try_from_slice_unchecked::<NoteAccountState>(&note.data.borrow()).unwrap();

    if *note_account.key != note_data.authority {
        return Err(NoteError::InvalidNoteAuthority.into());
    }

    note_data.title = title;
    note_data.body = body;
    note_data.serialize(&mut &mut note.data.borrow_mut()[..])?;

    Ok(())
}

pub fn delete_note(_program_id: &Pubkey, accounts: &[AccountInfo]) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let note_account = next_account_info(account_info_iter)?;
    let note = next_account_info(account_info_iter)?;

    let note_data = try_from_slice_unchecked::<NoteAccountState>(&note.data.borrow()).unwrap();

    if *note_account.key != note_data.authority {
        return Err(NoteError::InvalidNoteAuthority.into());
    }

    let noter_lamports = note_account.lamports();
    **note_account.lamports.borrow_mut() = noter_lamports.checked_add(note.lamports()).unwrap();
    **note.lamports.borrow_mut() = 0;

    let mut note = note.data.borrow_mut();
    note.fill(0);

    Ok(())
}
