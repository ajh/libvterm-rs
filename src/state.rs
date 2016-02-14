use libc::{c_int};

use super::*;

pub struct State {
    ptr: *mut ffi::VTermState
}

impl State {
    pub fn from_ptr(ptr: *mut ffi::VTermState) -> State {
        State { ptr: ptr }
    }

    pub fn color_to_index(&self, target: &ffi::VTermColor) -> u16 {
        for i in 0..256 {
            let color = self.get_palette_color(i);
            if color.red == target.red && color.green == target.green && color.blue == target.blue {
                return i as u16
            }
        }
        0
    }

    pub fn get_default_colors(&self) -> (ColorRGB, ColorRGB) {
        let mut fg_rgb: ffi::VTermColor = Default::default();
        let mut bg_rgb: ffi::VTermColor = Default::default();
        unsafe { ffi::vterm_state_get_default_colors(self.ptr, &mut fg_rgb, &mut bg_rgb) };

        (
            ColorRGB { red: fg_rgb.red, green: fg_rgb.green, blue: fg_rgb.blue },
            ColorRGB { red: bg_rgb.red, green: bg_rgb.green, blue: bg_rgb.blue },
        )
    }

    pub fn set_default_colors(&mut self, default_fg: ColorRGB, default_bg: ColorRGB) {
        let fg_rgb = ffi::VTermColor {
            red: default_fg.red,
            green: default_fg.green,
            blue: default_fg.blue
        };
        let bg_rgb = ffi::VTermColor {
            red: default_bg.red,
            green: default_bg.green,
            blue: default_bg.blue
        };

        unsafe { ffi::vterm_state_set_default_colors(self.ptr, &fg_rgb, &bg_rgb); };
    }

    pub fn get_palette_color(&self, index: u16) -> ColorRGB {
        let mut ffi_color: ffi::VTermColor = Default::default();
        unsafe { ffi::vterm_state_get_palette_color(self.ptr, index as c_int, &mut ffi_color); }
        ColorRGB {
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
    #[test]
    fn state_can_get_and_set_default_colors() {
        let mut vterm: ::VTerm = ::VTerm::new(::ScreenSize { rows: 2, cols: 2 });
        vterm.state.set_default_colors(::ColorRGB { red: 200, green: 201, blue: 202 },
                                       ::ColorRGB { red: 0, green: 1, blue: 2 });
        let (fg_rgb, bg_rgb) = vterm.state.get_default_colors();

        assert_eq!(fg_rgb.red, 200);
        assert_eq!(fg_rgb.green, 201);
        assert_eq!(fg_rgb.blue, 202);

        assert_eq!(bg_rgb.red, 0);
        assert_eq!(bg_rgb.green, 1);
        assert_eq!(bg_rgb.blue, 2);
    }
}
