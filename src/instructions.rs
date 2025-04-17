use crate::state::{Pool, Proposal};
use arch_program::{account::AccountInfo, program_error::ProgramError};
use borsh::BorshDeserialize;

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> Result<(), ProgramError> {
    let mut pool = Pool::try_from_slice(&accounts[0].data.borrow())?;
    match instruction_data[0] {
        0 => initialize_pool(&mut pool, accounts, &instruction_data[1..]),
        1 => contribute_bitcoin(&mut pool, accounts, &instruction_data[1..]),
        // Add other instruction handlers here
        _ => Err(ProgramError::InvalidInstructionData),
    }
}
