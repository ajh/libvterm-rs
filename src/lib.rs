#![feature(unique)]

#[macro_use]
extern crate log;
extern crate libc;
extern crate euclid;

use libc::c_int;

pub mod ffi;

mod vterm;
pub mod screen;
mod screen_cell;
mod state;
mod geom;

pub use vterm::*;
pub use screen_cell::*;
pub use state::*;
pub use geom::*;

#[derive(Debug)]
pub enum ScreenEvent {
    AltScreen {
        is_true: bool,
    },
    Bell,
    CursorBlink {
        is_true: bool,
    },
    CursorShape {
        value: usize,
    },
    CursorVisible {
        is_true: bool,
    },
    Damage {
        rect: Rect,
    },
    IconName {
        text: String,
    },
    Mouse {
        value: usize,
    },
    MoveCursor {
        new: Pos,
        old: Pos,
        is_visible: bool,
    },
    MoveRect {
        dest: Rect,
        src: Rect,
    },
    Resize {
        height: usize,
        width: usize,
    },
    Reverse {
        is_true: bool,
    },
    SbPopLine {
        cells: Vec<ScreenCell>,
    },
    SbPushLine {
        cells: Vec<ScreenCell>,
    },
    Title {
        text: String,
    },
}

pub fn int_to_bool(val: c_int) -> bool {
    match val {
        0 => false,
        _ => true,
    }
}

pub fn bool_to_int(flag: bool) -> c_int {
    match flag {
        true => -1,
        false => 0,
    }
}
