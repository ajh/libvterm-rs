#![feature(unique)]

#[macro_use]
extern crate log;
extern crate libc;
extern crate euclid;

use libc::c_int;

pub mod ffi;

mod events;
mod geom;
mod glyph_info;
mod screen;
mod screen_callbacks;
mod screen_cell;
mod state;
mod state_callbacks;
mod vterm;

pub use events::*;
pub use geom::*;
pub use glyph_info::*;
pub use screen::*;
pub use screen_cell::*;
pub use state::*;
pub use vterm::*;

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
