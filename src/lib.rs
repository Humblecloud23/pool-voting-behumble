mod instructions;
mod state;
mod utils;

use arch_program::entrypoint;
use instructions::process_instruction;

entrypoint!(process_instruction);
