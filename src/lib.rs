#![feature(libc)]

extern crate libc;

use libc::{c_int};

pub mod ffi;

mod vterm;
mod screen;
mod cell;
mod state;

pub use vterm::*;
pub use screen::*;
pub use cell::*;
pub use state::*;

pub fn int_to_bool(val: c_int) -> bool {
    match val {
        1 => true,
        _ => false,
    }
}

fn bool_to_int(flag: bool) -> c_int {
    match flag {
        true => 1,
        false => 0,
    }
}
