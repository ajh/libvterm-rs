use libc::{c_int, c_void};
use std::sync::mpsc;

use super::*;

pub enum StateEvent {
    PutGlyph {
        glyph_info: GlyphInfo,
        pos: Pos,
    },
    MoveCursor {
        new: Pos,
        old: Pos,
        is_visible: bool,
    },
    ScrollRect {
        rect: Rect,
        downward: isize,
        rightward: isize,
    },
    MoveRect {
        dest: Rect,
        src: Rect,
    },
    Erase {
        rect: Rect,
        /// ?
        selective: isize,
    },
    InitPen,
    /// Need to flesh this out still
    SetTermProp,
    Bell,
    Resize,
    SetLineInfo,
}

pub struct StateCallbacksConfig {
    pub put_glyph: bool,
    pub move_cursor: bool,
    pub scroll_rect: bool,
    pub move_rect: bool,
    pub erase: bool,
    pub init_pen: bool,
    pub set_pen_attr: bool,
    pub set_term_prop: bool,
    pub bell: bool,
    pub resize: bool,
    pub set_line_info: bool,
}

impl StateCallbacksConfig {
    pub fn all() -> StateCallbacksConfig {
        StateCallbacksConfig {
            put_glyph: true,
            move_cursor: true,
            scroll_rect: true,
            move_rect: true,
            erase: true,
            init_pen: true,
            set_pen_attr: true,
            set_term_prop: true,
            bell: true,
            resize: true,
            set_line_info: true,
        }
    }

    pub fn none() -> StateCallbacksConfig {
        StateCallbacksConfig {
            put_glyph: false,
            move_cursor: false,
            scroll_rect: false,
            move_rect: false,
            erase: false,
            init_pen: false,
            set_pen_attr: false,
            set_term_prop: false,
            bell: false,
            resize: false,
            set_line_info: false,
        }
    }
}

impl VTerm {
    pub fn state_get_default_colors(&self) -> (ColorRGB, ColorRGB) {
        let mut fg_rgb: ffi::VTermColor = Default::default();
        let mut bg_rgb: ffi::VTermColor = Default::default();
        unsafe {
            ffi::vterm_state_get_default_colors(self.state_ptr.get(), &mut fg_rgb, &mut bg_rgb)
        };

        (ColorRGB {
            red: fg_rgb.red,
            green: fg_rgb.green,
            blue: fg_rgb.blue,
        },
         ColorRGB {
            red: bg_rgb.red,
            green: bg_rgb.green,
            blue: bg_rgb.blue,
        })
    }

    pub fn state_set_default_colors(&mut self, default_fg: &ColorRGB, default_bg: &ColorRGB) {
        let fg_rgb = ffi::VTermColor {
            red: default_fg.red,
            green: default_fg.green,
            blue: default_fg.blue,
        };
        let bg_rgb = ffi::VTermColor {
            red: default_bg.red,
            green: default_bg.green,
            blue: default_bg.blue,
        };

        unsafe {
            ffi::vterm_state_set_default_colors(self.state_ptr.get_mut(), &fg_rgb, &bg_rgb);
        };
    }

    pub fn state_get_rgb_color_from_palette(&self, index: usize) -> ColorRGB {
        let mut ffi_color: ffi::VTermColor = Default::default();
        unsafe {
            ffi::vterm_state_get_palette_color(self.state_ptr.get(),
                                               index as c_int,
                                               &mut ffi_color);
        }
        ColorRGB {
            red: ffi_color.red,
            green: ffi_color.green,
            blue: ffi_color.blue,
        }
    }

    pub fn state_get_palette_color_from_rgb(&self, target: &ColorRGB) -> usize {
        for i in 0..256 {
            let color = self.state_get_rgb_color_from_palette(i);
            if color.red == target.red && color.green == target.green && color.blue == target.blue {
                return i as usize;
            }
        }
        0
    }

    /// move this to ffi classes since it deals with the ffi color type
    pub fn state_get_palette_color_from_c_rgb(&self, target: &ffi::VTermColor) -> usize {
        for i in 0..256 {
            let color = self.state_get_rgb_color_from_palette(i);
            if color.red == target.red && color.green == target.green && color.blue == target.blue {
                return i as usize;
            }
        }
        0
    }

    pub fn state_reset(&mut self, hard: bool) {
        unsafe {
            ffi::vterm_state_reset(self.state_ptr.get_mut(), ::bool_to_int(hard));
        }
    }

    pub fn state_receive_events(&mut self, config: &StateCallbacksConfig) {
        let mut callbacks: ffi::VTermStateCallbacks = Default::default();

        // Note: Some of these seem to be required or libvterm will segfault
        callbacks.put_glyph = if config.put_glyph { Some(::state_callbacks::put_glyph) } else { None };
        callbacks.move_cursor = if config.move_cursor { Some(::state_callbacks::move_cursor) } else { None };
        callbacks.scroll_rect = if config.scroll_rect { Some(::state_callbacks::scroll_rect) } else { None };
        callbacks.move_rect = if config.move_rect { Some(::state_callbacks::move_rect) } else { None };
        callbacks.erase = if config.erase { Some(::state_callbacks::erase) } else { None };
        callbacks.init_pen = if config.init_pen { Some(::state_callbacks::init_pen) } else { None };
        callbacks.set_pen_attr = if config.set_pen_attr { Some(::state_callbacks::set_pen_attr) } else { None };
        callbacks.set_term_prop = if config.set_term_prop { Some(::state_callbacks::set_term_prop) } else { None };
        callbacks.bell = if config.bell { Some(::state_callbacks::bell) } else { None };
        callbacks.resize = if config.resize { Some(::state_callbacks::resize) } else { None };
        callbacks.set_line_info = if config.set_line_info { Some(::state_callbacks::set_line_info) } else { None };

        self.state_callbacks = Some(callbacks);

        if self.state_event_tx.is_none() {
            let (tx, rx) = mpsc::channel();
            self.state_event_tx = Some(tx);
            self.state_event_rx = Some(rx);
        }

        unsafe {
            let self_ptr: *mut c_void = self as *mut _ as *mut c_void;
            ffi::vterm_state_set_callbacks(self.state_ptr.get_mut(),
                                            self.state_callbacks.as_ref().unwrap(),
                                            self_ptr);
        }
    }
}

mod tests {
    #![allow(unused_imports)]
    use super::super::*;
    use std::io::prelude::*;

    #[test]
    fn state_can_get_and_set_default_colors() {
        let mut vterm: VTerm = VTerm::new(&Size {
            height: 2,
            width: 2,
        });
        vterm.state_set_default_colors(&ColorRGB {
                                           red: 200,
                                           green: 201,
                                           blue: 202,
                                       },
                                       &ColorRGB {
                                           red: 0,
                                           green: 1,
                                           blue: 2,
                                       });
        let (fg_rgb, bg_rgb) = vterm.state_get_default_colors();

        assert_eq!(fg_rgb.red, 200);
        assert_eq!(fg_rgb.green, 201);
        assert_eq!(fg_rgb.blue, 202);

        assert_eq!(bg_rgb.red, 0);
        assert_eq!(bg_rgb.green, 1);
        assert_eq!(bg_rgb.blue, 2);
    }
}
