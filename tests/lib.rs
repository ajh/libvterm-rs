extern crate vterm_sys;
extern crate regex;

use regex::Regex;
use vterm_sys::*;

#[test]
fn screen_can_get_text() {
    let mut vterm: VTerm = VTerm::new(ScreenSize { rows: 2, cols: 2 });
    vterm.write(b"hi");

    let text = vterm.screen.get_text(Rect { start_row: 0, end_row: 2, start_col: 0, end_col: 2 });
    let re = Regex::new(r"hi").unwrap();
    assert!(re.is_match(&text));
}
