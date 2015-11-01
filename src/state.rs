extern crate libc;

use libc::{c_int};

use super::*;

pub struct VTermState {
    ptr: *mut ffi::VTermState
}

impl VTermState {
    pub fn from_ptr(ptr: *mut ffi::VTermState) -> VTermState {
        VTermState { ptr: ptr }
    }

    pub fn get_default_colors(&self) -> (VTermColor, VTermColor) {
        let mut fg: ffi::VTermColor = Default::default();
        let mut bg: ffi::VTermColor = Default::default();
        unsafe { ffi::vterm_state_get_default_colors(self.ptr, &mut fg, &mut bg) };

        (
            VTermColor { red: fg.red, green: fg.green, blue: fg.blue },
            VTermColor { red: bg.red, green: bg.green, blue: bg.blue },
        )
    }
}

mod tests {
    use super::super::*;

    #[test]
    fn state_can_get_default_colors() {
        let mut vterm: VTerm = VTerm::new(2, 2);
        let state = vterm.get_state();
        state.get_default_colors();
    }
}
