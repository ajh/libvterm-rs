extern crate libc;

use libc::{c_int};

use super::*;

pub struct State {
    ptr: *mut ffi::VTermState
}

impl State {
    pub fn from_ptr(ptr: *mut ffi::VTermState) -> State {
        State { ptr: ptr }
    }

    pub fn get_default_colors(&self) -> (Color, Color) {
        let mut fg: ffi::VTermColor = Default::default();
        let mut bg: ffi::VTermColor = Default::default();
        unsafe { ffi::vterm_state_get_default_colors(self.ptr, &mut fg, &mut bg) };

        (
            Color { red: fg.red, green: fg.green, blue: fg.blue },
            Color { red: bg.red, green: bg.green, blue: bg.blue },
        )
    }

    pub fn get_palette_color(&self, index: u16) -> Color {
        let mut ffi_color: ffi::VTermColor = Default::default();
        unsafe { ffi::vterm_state_get_palette_color(self.ptr, index as c_int, &mut ffi_color); }
        Color {
            red: ffi_color.red,
            green: ffi_color.green,
            blue: ffi_color.blue,
        }
    }
}

mod tests {
    use super::super::*;

    #[test]
    fn state_can_get_default_colors() {
        let vterm: VTerm = VTerm::new(2, 2);
        vterm.state.get_default_colors();
    }
}
