use std::mem::replace;

///
/// Input Buffer
///
#[derive(Debug)]
pub struct InputBuffer {
    buffer: [bool; 16],
}

impl InputBuffer {
    #[inline]
    pub fn new() -> Self {
        Self {
            buffer: [false; 16],
        }
    }

    #[inline]
    pub fn get_key(&self, key: usize) -> &bool {
        &self.buffer[key]
    }

    #[inline]
    pub fn get_buffer(&mut self) -> &mut [bool; 16] {
        &mut self.buffer
    }

    #[inline]
    pub fn _set(&mut self, key: usize, is_pressed: bool) {
        let _ = replace(&mut self.buffer[key], is_pressed);
    }

    #[inline]
    pub fn clear(&mut self) {
        let _ = replace(&mut self.buffer, [false; 16]);
    }
}
