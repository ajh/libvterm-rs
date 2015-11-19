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

    pub fn set_default_colors(&mut self, default_fg: Color, default_bg: Color) {
        let mut fg = ffi::VTermColor {
            red: default_fg.red,
            green: default_fg.green,
            blue: default_fg.blue
        };
        let mut bg = ffi::VTermColor {
            red: default_bg.red,
            green: default_bg.green,
            blue: default_bg.blue
        };

        unsafe { ffi::vterm_state_set_default_colors(self.ptr, &fg, &bg); };
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

    pub fn reset(&mut self, hard: bool) {
      unsafe { ffi::vterm_state_reset(self.ptr, ::bool_to_int(hard)); }
    }
}

mod tests {
    use super::super::*;

    #[test]
    fn state_can_get_and_set_default_colors() {
        let mut vterm: VTerm = VTerm::new(ScreenSize { rows: 2, cols: 2 });
        vterm.state.set_default_colors(Color { red: 200, green: 201, blue: 202 },
                                       Color { red: 0, green: 1, blue: 2 });
        let (fg, bg) = vterm.state.get_default_colors();

        assert_eq!(fg.red, 200);
        assert_eq!(fg.green, 201);
        assert_eq!(fg.blue, 202);

        assert_eq!(bg.red, 0);
        assert_eq!(bg.green, 1);
        assert_eq!(bg.blue, 2);
    }
}
