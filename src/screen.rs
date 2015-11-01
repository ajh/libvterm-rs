extern crate libc;

use libc::{c_int};

use super::*;

pub struct ScreenSize {
    pub rows: usize,
    pub cols: usize,
}

pub struct Pos {
    pub row: usize,
    pub col: usize,
}

pub struct Screen {
    ptr: *mut ffi::VTermScreen
}

impl Screen {
    pub fn from_ptr(ptr: *mut ffi::VTermScreen) -> Screen {
        Screen { ptr: ptr }
    }

    pub fn reset(&mut self, is_hard: bool) {
        unsafe { ffi::vterm_screen_reset(self.ptr, super::bool_to_int(is_hard)) }
    }

    //pub fn get_cell(&self, pos: Pos) -> VTermCell
}

mod tests {
    use super::super::*;

    #[test]
    fn screen_can_reset() {
        let vterm: VTerm = VTerm::new(2, 2);
        let mut screen = vterm.get_screen();
        screen.reset(true);
    }
}
