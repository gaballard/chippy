pub mod sprite;

use std::{
    collections::HashMap,
    mem::{replace, swap},
};

use log::debug;

use crate::{
    chip8::sprite::{Sprite, SpriteKey},
    chippy::InstructionSet,
    constants,
    utils::{get_display_height, get_display_width, read_bit_from_byte},
};

#[derive(Debug, PartialEq)]
pub enum ScrollDir {
    Up,
    Down,
    Left,
    Right,
}

///
/// Video Memory
///
#[derive(Debug)]
#[must_use]
pub struct VideoMemory {
    instruction_set: InstructionSet,
    pub hires_mode: bool,
    pub vram_changed: bool,
    data: [[u8; constants::HIRES_SCREEN_WIDTH]; constants::HIRES_SCREEN_HEIGHT],
    sprites: HashMap<SpriteKey, Sprite>,
    collisions: u8,
}

impl VideoMemory {
    pub fn new(instruction_set: InstructionSet) -> Self {
        Self {
            instruction_set,
            hires_mode: false,
            vram_changed: false,
            data: [[0; constants::HIRES_SCREEN_WIDTH]; constants::HIRES_SCREEN_HEIGHT],
            sprites: HashMap::new(),
            collisions: 0,
        }
    }

    ///
    /// Memory
    ///

    fn get_data_pos(&self, x: usize, y: usize) -> (usize, usize) {
        let display_width = get_display_width(&self.instruction_set);
        let display_height = get_display_height(&self.instruction_set);

        let dx = if x >= display_width as usize {
            display_width as usize - 1
        } else {
            x
        };

        let dy = if y >= display_height as usize {
            display_height as usize - 1
        } else {
            y
        };

        (dx, dy)
    }

    pub fn read(&self, x: usize, y: usize) -> &u8 {
        let (dx, dy) = self.get_data_pos(x, y);
        &self.data[dy][dx]
    }

    pub fn write(&mut self, x: usize, y: usize, value: u8) {
        let (dx, dy) = self.get_data_pos(x, y);
        let _ = replace(&mut self.data[dy][dx], value);
        self.vram_changed = true;
    }

    pub fn clear(&mut self) {
        let _ = replace(
            &mut self.data,
            [[0; constants::HIRES_SCREEN_WIDTH]; constants::HIRES_SCREEN_HEIGHT],
        );
        self.vram_changed = true;
    }

    ///
    /// Scrolling
    ///

    pub fn scroll(&mut self, dir: ScrollDir, amount: usize) {
        // let mut new_data = [[0; constants::HIRES_SCREEN_WIDTH]; constants::HIRES_SCREEN_HEIGHT];
        let mut new_data = self.data.clone();
        let data_length = new_data.len();

        let display_height = get_display_height(&self.instruction_set) as usize;
        let display_width = get_display_width(&self.instruction_set) as usize;

        // let mut collisions: u32 = 0;

        // match dir {
        //     ScrollDir::Down => {
        //         let to_move = &mut new_data[0..data_length - amount];
        //         let to_clip = &new_data[data_length - amount..];

        //         for row in to_clip {
        //             for p in row {
        //                 if *p == 1 {
        //                     collisions += 1;
        //                 }
        //             }
        //         }

        //         new_data[0..data_length - amount].swap_with_slice(to_move);

        //         // let _ = replace(&mut new_data[0..self.data.len() - amount], to_move);
        //         // new_data[0..self.data.len() - amount] = to_move;
        //     }
        //     _ => {}
        // }

        // down:
        //  - get last `amount` rows
        //  - check those rows for pixels and add 1 to the collisions counter
        //  - get the first `len - amount` rows

        // TODO: update the sprite positions as well (or tie this into sprites in general?)

        for y in 0..display_height {
            for x in 0..display_width {
                let new_val = match dir {
                    ScrollDir::Down => self.data[y.saturating_sub(amount)][x],
                    ScrollDir::Up => {
                        self.data[if y + amount >= display_height {
                            display_height - 1
                        } else {
                            y + amount
                        }][x]
                    }
                    ScrollDir::Right => self.data[y][x.saturating_sub(amount)],
                    ScrollDir::Left => {
                        self.data[y][if x + amount >= display_width {
                            display_width - 1
                        } else {
                            x + amount
                        }]
                    }
                };
                new_data[y][x] = new_val;
            }
        }

        swap(&mut self.data, &mut new_data);

        // collisions
    }

    ///
    /// Sprite Handling
    ///

    pub fn has_sprite(&self, key: &SpriteKey) -> bool {
        match self.get_sprite(key) {
            Some(_) => true,
            _ => false,
        }
    }

    pub fn get_sprite(&self, key: &SpriteKey) -> Option<&Sprite> {
        self.sprites.get(key)
    }

    pub fn get_sprite_mut(&mut self, key: &SpriteKey) -> Option<&mut Sprite> {
        self.sprites.get_mut(key)
    }

    pub fn add_sprite(&mut self, key: &SpriteKey, width: u8, height: u8, sprite_data: &[u8]) {
        self.sprites
            .insert(*key, Sprite::new(*key, width, height, sprite_data.to_vec()));
    }

    pub fn place_sprite(&mut self, key: &SpriteKey, x: usize, y: usize) {
        let display_width = get_display_width(&self.instruction_set) as usize;
        let display_height = get_display_height(&self.instruction_set) as usize;

        let px = if x >= display_width {
            x % display_width
        } else {
            x
        };
        let py = if y >= display_height {
            y % display_height
        } else {
            y
        };

        match self.get_sprite_mut(key) {
            Some(sprite) => sprite.set_coords(px, py),
            _ => return,
        }
    }

    pub fn draw_sprite(&mut self, key: &SpriteKey) -> u8 {
        let mut collisions: u8 = 0;

        let sprite = match self.get_sprite(key) {
            Some(sprite) => sprite.to_owned(),
            _ => {
                debug!("Couldn't find sprite with key {:?}", key);
                return 0;
            }
        };

        for sy in 0..sprite.get_height() {
            let vy = sprite.get_y().saturating_add(sy as usize);
            for sx in 0..sprite.get_width() {
                let vx = sprite.get_x().saturating_add(sx as usize);

                let pixel = read_bit_from_byte(
                    sprite.get_data()[sy as usize],
                    (7 as u8).wrapping_sub(sx as u8),
                );

                if pixel == 0 {
                    continue;
                }

                if self.read(vx, vy) & pixel == 1 {
                    collisions += 1;
                }

                self.write(vx, vy, self.read(vx, vy) ^ pixel);
            }
        }

        self.collisions = collisions;

        collisions
    }

    pub fn clear_collisions(&mut self) {
        self.collisions = 0;
    }
}
