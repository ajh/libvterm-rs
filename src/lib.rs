#![feature(unique)]

#[macro_use]
extern crate log;
extern crate libc;

use libc::c_int;

pub mod ffi;

mod vterm;
pub mod screen;
mod screen_cell;
mod state;

pub use vterm::*;
pub use screen_cell::*;
pub use state::*;

#[derive(Debug, Default, Clone, PartialEq)]
pub struct ScreenSize {
    pub rows: usize,
    pub cols: usize,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pos {
    pub row: usize,
    pub col: usize,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Rect {
    pub start_row: usize,
    pub end_row: usize,
    pub start_col: usize,
    pub end_col: usize,
}

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
        rows: usize,
        cols: usize,
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
