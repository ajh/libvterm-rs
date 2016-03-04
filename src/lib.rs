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

pub use vterm::*;
pub use screen_cell::*;
pub use state::*;

pub type Size = euclid::Size2D<usize>;
pub type Pos = euclid::Point2D<usize>;
pub type Rect = euclid::Rect<usize>;

pub trait RectAssist {
    fn top(&self) -> usize;
    fn left(&self) -> usize;
    fn bottom(&self) -> usize;
    fn right(&self) -> usize;

    fn positions<'a>(&'a self) -> RectPositions<'a>;
}

impl RectAssist for Rect {
    fn top(&self) -> usize {
        self.origin.y
    }
    fn left(&self) -> usize {
        self.origin.x
    }
    fn bottom(&self) -> usize {
        self.origin.y + self.size.height
    }
    fn right(&self) -> usize {
        self.origin.x + self.size.width
    }

    fn positions<'a>(&'a self) -> RectPositions<'a> {
        RectPositions { rect: self }
    }
}

// TODO:
//
// * implement Iterator trait with tests
// * move geometry stuff to its own geom.rs file
//
pub struct RectPositions<'a> {
    pub rect: &'a Rect
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
