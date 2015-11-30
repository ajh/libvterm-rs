#![feature(libc)]
#![feature(unicode)]
#![feature(convert)]

#[macro_use]
extern crate log;
extern crate libc;

use libc::{c_int};

pub mod ffi;

mod vterm;
mod screen;
mod screen_cell;
mod state;

pub use vterm::*;
pub use screen::*;
pub use screen_cell::*;
pub use state::*;

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
