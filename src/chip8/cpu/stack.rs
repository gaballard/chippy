use std::mem::replace;

///
/// Stack
///
#[derive(Debug)]
pub struct Stack {
    data: [u16; 16],
    pointer: u8,
}

impl Stack {
    #[inline]
    pub fn new() -> Self {
        Self {
            data: [0; 16],
            pointer: 0,
        }
    }

    #[inline]
    pub fn push(&mut self, value: u16) {
        let new_pointer = self.pointer + 1;
        let _ = replace(&mut self.pointer, new_pointer);
        let _ = replace(&mut self.data[self.pointer as usize], value);
    }

    #[inline]
    pub fn pop(&mut self) -> u16 {
        let addr = replace(&mut self.data[self.pointer as usize], 0);
        let new_pointer = self.pointer - 1;
        let _ = replace(&mut self.pointer, new_pointer);

        addr
    }

    #[inline]
    pub fn clear(&mut self) {
        let _ = replace(&mut self.data, [0; 16]);
        let _ = replace(&mut self.pointer, 0);
    }
}
