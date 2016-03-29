mod cell;
mod parser;
mod screen;
mod state;
mod vterm;
mod glyph_info;

pub use self::cell::*;
pub use self::parser::*;
pub use self::screen::*;
pub use self::state::*;
pub use self::vterm::*;
pub use self::glyph_info::*;

use libc::{c_int, uint8_t};

#[derive(Debug)]
#[repr(C)]
pub enum VTermProp {
    VTermPropCursorVisible = 1, // bool
    VTermPropCursorBlink, // bool
    VTermPropAltscreen, // bool
    VTermPropTitle, // string
    VTermPropIconName, // string
    VTermPropReverse, // bool
    VTermPropCursorShape, // number
    VTermPropMouse, // number
}

pub enum VTermValue {}

#[repr(C)]
#[derive(PartialEq, Debug)]
pub struct VTermPos {
    pub row: c_int,
    pub col: c_int,
}

impl VTermPos {
    pub fn from_pos(pos: &::Pos) -> VTermPos {
        VTermPos {
            col: pos.x as i32,
            row: pos.y as i32,
        }
    }

    pub fn as_pos(&self) -> ::Pos {
        ::Pos {
            x: self.col as usize,
            y: self.row as usize,
        }
    }
}

#[repr(C)]
#[derive(PartialEq, Debug, Clone, Default)]
pub struct VTermColor {
    pub red: uint8_t,
    pub green: uint8_t,
    pub blue: uint8_t,
}

impl VTermColor {
    pub fn as_color_rgb(&self) -> ::ColorRGB {
        ::ColorRGB {
            red: self.red,
            green: self.green,
            blue: self.blue,
        }
    }
}


#[repr(C)]
#[derive(PartialEq, Debug)]
pub struct VTermRect {
    // End values are exclusive, meaning the maximum index is the end_size - 1.
    pub start_row: c_int,
    pub end_row: c_int,
    pub start_col: c_int,
    pub end_col: c_int,
}

impl VTermRect {
    pub fn from_rect(rect: &::Rect) -> VTermRect {
        VTermRect {
            start_col: rect.origin.x as i32,
            start_row: rect.origin.y as i32,
            end_col: (rect.origin.x + rect.size.width) as i32,
            end_row: (rect.origin.y + rect.size.height) as i32,
        }
    }

    pub fn as_rect(&self) -> ::Rect {
        ::Rect {
            origin: ::Pos {
                x: self.start_col as usize,
                y: self.start_row as usize,
            },
            size: ::Size {
                width: (self.end_col - self.start_col) as usize,
                height: (self.end_row - self.start_row) as usize,
            },
        }
    }
}

#[derive(Debug)]
#[repr(C)]
pub enum VTermAttr {
    Bold = 1, // bool:   1, 22
    Underline, // number: 4, 21, 24
    Italic, // bool:   3, 23
    Blink, // bool:   5, 25
    Reverse, // bool:   7, 27
    Strike, // bool:   9, 29
    Font, // number: 10-19
    Foreground, // color:  30-39 90-97
    Background, // color:  40-49 100-107
}

#[repr(C)]
pub enum VTermModifier {
    VTERM_MOD_NONE = 0x00,
    VTERM_MOD_SHIFT = 0x01,
    VTERM_MOD_ALT = 0x02,
    VTERM_MOD_CTRL = 0x04,
}

#[repr(C)]
pub enum VTermKey {
    VTERM_KEY_NONE,

    VTERM_KEY_ENTER,
    VTERM_KEY_TAB,
    VTERM_KEY_BACKSPACE,
    VTERM_KEY_ESCAPE,

    VTERM_KEY_UP,
    VTERM_KEY_DOWN,
    VTERM_KEY_LEFT,
    VTERM_KEY_RIGHT,

    VTERM_KEY_INS,
    VTERM_KEY_DEL,
    VTERM_KEY_HOME,
    VTERM_KEY_END,
    VTERM_KEY_PAGEUP,
    VTERM_KEY_PAGEDOWN,

    VTERM_KEY_FUNCTION_0 = 256,
    VTERM_KEY_FUNCTION_MAX = 256 + 255,

    VTERM_KEY_KP_0,
    VTERM_KEY_KP_1,
    VTERM_KEY_KP_2,
    VTERM_KEY_KP_3,
    VTERM_KEY_KP_4,
    VTERM_KEY_KP_5,
    VTERM_KEY_KP_6,
    VTERM_KEY_KP_7,
    VTERM_KEY_KP_8,
    VTERM_KEY_KP_9,
    VTERM_KEY_KP_MULT,
    VTERM_KEY_KP_PLUS,
    VTERM_KEY_KP_COMMA,
    VTERM_KEY_KP_MINUS,
    VTERM_KEY_KP_PERIOD,
    VTERM_KEY_KP_DIVIDE,
    VTERM_KEY_KP_ENTER,
    VTERM_KEY_KP_EQUAL,

    VTERM_KEY_MAX, // Must be last
}
