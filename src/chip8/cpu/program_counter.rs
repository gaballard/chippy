use std::mem::{replace, swap};

use crate::constants;

///
/// Program Counter
///
#[derive(Debug)]
pub struct ProgramCounter {
    pub address: u16,
}

impl ProgramCounter {
    #[inline]
    pub const fn new() -> Self {
        ProgramCounter {
            address: constants::PROGRAM_START_ADDR,
        }
    }

    #[inline]
    pub fn next(&mut self) {
        let mut next_addr = self.address.wrapping_add(constants::OPCODE_SIZE);
        swap(&mut self.address, &mut next_addr);
    }

    #[inline]
    pub fn jump(&mut self, jump_addr: u16) {
        let _ = replace(&mut self.address, jump_addr);
    }

    #[inline]
    pub fn reset(&mut self) {
        let _ = replace(&mut self.address, constants::PROGRAM_START_ADDR);
    }
}
