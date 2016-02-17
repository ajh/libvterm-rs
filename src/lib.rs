#![feature(unique)]

#[macro_use]
extern crate log;
extern crate libc;

use libc::{c_int};

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
    pub rows: u16,
    pub cols: u16,
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Pos {
    /// negative numbers represent scroll buffer positions
    pub row: i16,
    pub col: i16,
}

#[derive(Debug, Default, PartialEq)]
pub struct Rect {
    pub start_row: u16,
    pub end_row: u16,
    pub start_col: u16,
    pub end_col: u16,
}

#[derive(Debug)]
pub enum ScreenEvent {
    AltScreen     { is_true: bool },
    Bell,
    CursorBlink   { is_true: bool },
    CursorShape   { value: isize },
    CursorVisible { is_true: bool },
    Damage        { rect: Rect },
    IconName      { text: String},
    Mouse         { value: isize },
    MoveCursor    { new: Pos,                  old: Pos,       is_visible: bool },
    MoveRect      { dest: Rect,                src: Rect },
    Resize        { rows: u16,               cols: u16 },
    Reverse       { is_true: bool },
    SbPopLine     { cells: Vec<ScreenCell> },
    SbPushLine    { cells: Vec<ScreenCell> },
    Title         { text: String},
}


pub fn int_to_bool(val: c_int) -> bool {
    match val {
        0 => false,
        _ => true,
    }
}

fn bool_to_int(flag: bool) -> c_int {
    match flag {
        true => -1,
        false => 0,
    }
}
