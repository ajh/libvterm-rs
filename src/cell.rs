extern crate libc;

use libc::{c_int};
use std::char;
use std::vec::Vec;

use super::*;

pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

pub struct Cell {
    ptr: *mut ffi::VTermScreenCell
}

impl Cell {
    pub fn from_ptr(ptr: *mut ffi::VTermScreenCell) -> Cell {
        Cell { ptr: ptr }
    }

    pub fn new() -> Cell {
        let ptr = unsafe { ffi::vterm_cell_new() };
        Cell { ptr: ptr }
    }

    pub fn get_width(&self) -> usize {
        unsafe { ffi::vterm_cell_get_width(self.ptr) as usize }
    }

    pub fn get_chars(&self) -> Vec<char> {
        let mut buf = [0 as libc::uint32_t; ffi::VTERM_MAX_CHARS_PER_CELL];
        let ret = unsafe { ffi::vterm_cell_get_chars(self.ptr, buf.as_mut_ptr(), ffi::VTERM_MAX_CHARS_PER_CELL as u64) };
        println!("{}", ret);

        let mut output: Vec<char> = Vec::with_capacity(ffi::VTERM_MAX_CHARS_PER_CELL);
        for i in 0..(ret as usize) {
            let ch = char::from_u32(buf[i]).unwrap();
            output.push(ch);
        }

        output
    }

    pub fn get_chars_utf8_encoded_as_bytes(&self) -> Vec<u8> {
        const MAX_BYTES_PER_UTF8_CHAR: usize = 4; // I read this on the internet somewhere ;)
        let mut output: Vec<u8> = Vec::with_capacity(ffi::VTERM_MAX_CHARS_PER_CELL * MAX_BYTES_PER_UTF8_CHAR);

        for ch in self.get_chars() {
            let mut bytes = [0 as u8; MAX_BYTES_PER_UTF8_CHAR];
            match ch.encode_utf8(&mut bytes) {
                Some(size) => {
                    for byte in &bytes[0..size] {
                        output.push(*byte);
                    }
                },
                None => panic!("char couldn't be encoded as utf8: {}", ch),
            }
        }

        output
    }
}

impl Drop for Cell {
    fn drop(&mut self) {
        unsafe { ffi::vterm_cell_free(self.ptr) }
    }
}

mod tests {
    use super::super::*;

    #[test]
    fn cell_can_create_and_destroy() {
        let cell = Cell::new();
        drop(cell);
    }

    #[test]
    fn cell_can_get_chars() {
        let mut vt = VTerm::new(2,2);
        // This doesn't work. I think the callbacks are supposed to be in place.
        //let input = b"hi!";
        //vt.write(input);

        let cell = vt.get_screen().get_cell(&Pos { row: 0, col: 1 });
        cell.get_chars();
        //let chs = cell.get_chars();
        //assert_eq!(1, chs.len());
        //assert_eq!('i', chs[0]);
    }

    #[test]
    fn cell_can_get_chars_utf8_encoded_as_bytes() {
        let mut vt = VTerm::new(2,2);
        // This doesn't work. I think the callbacks are supposed to be in place.
        //let input = b"hi!";
        //vt.write(input);

        let cell = vt.get_screen().get_cell(&Pos { row: 0, col: 1 });
        cell.get_chars_utf8_encoded_as_bytes();
        //let chs = cell.get_chars_utf8_encoded_as_bytes();
        //assert_eq!(1, chs.len());
        //assert_eq!(b'i', chs[0]);
    }
}
