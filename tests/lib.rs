extern crate libc;
extern crate regex;
extern crate term;
extern crate vterm_sys;

mod rs;
mod support;
//mod ffi;

use regex::Regex;
use vterm_sys::*;
use std::io::prelude::*;

#[test]
fn screen_can_get_text() {
    let mut vterm: VTerm = VTerm::new(&Size { height: 2, width: 2 });
    vterm.write(b"hi").unwrap();

    let text = vterm.screen_get_text_lossy(&Rect::new(Pos {
        x: 0,
        y: 0,
    },
    Size {
        width: 2,
        height: 1,
    }));
    let re = Regex::new(r"hi").unwrap();
    assert!(re.is_match(&text));
}
