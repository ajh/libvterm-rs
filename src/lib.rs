#![feature(libc)]
#![feature(unicode)]

extern crate libc;

use libc::{c_int};

macro_rules! warn {
    ($($arg:tt)*) => (
        {
            use std::io::prelude::*;
            if let Err(e) = write!(&mut ::std::io::stderr(), "{}", format_args!($($arg)*)) {
                panic!("Failed to write to stderr.\
                    \nOriginal error output: {}\
                    \nSecondary error writing to stderr: {}", format!($($arg)*), e);
            }
        }
    )
}

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
