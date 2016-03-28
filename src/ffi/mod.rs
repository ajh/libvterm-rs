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

use libc::c_int;

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
