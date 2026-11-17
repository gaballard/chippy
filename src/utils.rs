use crate::{chippy::InstructionSet, constants};

///
/// Returns an individual bit from a given byte
///
#[inline]
pub fn read_bit_from_byte(byte: u8, bit_position: u8) -> u8 {
    if bit_position < 8 {
        if byte & (1 << bit_position) != 0 {
            1
        } else {
            0
        }
    } else {
        0
    }
}

pub fn get_display_width(instruction_set: &InstructionSet) -> u32 {
    (if instruction_set.is_schip() {
        constants::HIRES_SCREEN_WIDTH
    } else if *instruction_set == InstructionSet::Chip8III {
        constants::CHIP8III_SCREEN_WIDTH
    } else {
        constants::LORES_SCREEN_WIDTH
    }) as u32
}

pub fn get_display_height(instruction_set: &InstructionSet) -> u32 {
    (if instruction_set.is_schip() {
        constants::HIRES_SCREEN_HEIGHT
    } else if *instruction_set == InstructionSet::Chip8III {
        constants::CHIP8III_SCREEN_HEIGHT
    } else {
        constants::LORES_SCREEN_HEIGHT
    }) as u32
}
