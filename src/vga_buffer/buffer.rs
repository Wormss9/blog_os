use core::ptr;

use super::ScreenChar;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl Buffer {
    pub fn new(pointer: usize) -> &'static mut Self {
        unsafe { &mut *(pointer as *mut Buffer) }
    }
    pub fn read(&self, row: usize, col: usize) -> ScreenChar {
        unsafe { ptr::read_volatile(&self.chars[row][col]) }
    }
    pub fn write(&mut self, row: usize, col: usize, character: ScreenChar) {
        unsafe {
            ptr::write_volatile(&mut (*self).chars[row][col], character);
        }
    }
}
