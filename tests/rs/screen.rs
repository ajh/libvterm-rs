use std::io::prelude::*;
use vterm_sys::*;

#[test]
fn screen_get_cell_returns_cell() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 1,
        width: 1,
    });
    let cell = vterm.screen_get_cell(&Pos::new(0, 0));
    assert_eq!(cell.chars.len(), 0);

    vterm.write(b"a").unwrap();
    vterm.flush().unwrap();

    let cell = vterm.screen_get_cell(&Pos::new(0, 0));
    assert_eq!(cell.chars[0], b'a');
}

#[test]
#[should_panic]
fn screen_get_cell_panics_if_out_of_bounds() {
    let vterm: VTerm = VTerm::new(&Size {
        height: 1,
        width: 1,
    });
    vterm.screen_get_cell(&Pos::new(1, 0));
}

#[test]
fn screen_get_text_lossy_returns_string() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.write(b"abcd").unwrap();
    vterm.flush().unwrap();

    assert_eq!(vterm.screen_get_text_lossy(&Rect::new(Pos::new(0, 0), Size::new(2, 2))),
               "ab\ncd");
}

#[test]
fn screen_get_text_lossy_returns_string_from_part_of_screen() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.write(b"abcd").unwrap();
    vterm.flush().unwrap();

    assert_eq!(vterm.screen_get_text_lossy(&Rect::new(Pos::new(1, 0), Size::new(1, 2))),
               "b\nd");
}

#[test]
#[should_panic]
fn screen_get_text_lossy_panics_if_rect_is_out_of_bounds() {
    let vterm: VTerm = VTerm::new(&Size {
        height: 1,
        width: 1,
    });
    vterm.screen_get_text_lossy(&Rect::new(Pos::new(1, 0), Size::new(1, 2)));
}

#[test]
fn screen_get_text_returns_string() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.write(b"abcd").unwrap();
    vterm.flush().unwrap();

    assert_eq!(vterm.screen_get_text(&Rect::new(Pos::new(0, 0), Size::new(2, 2))).unwrap(),
               "ab\ncd");
}

#[test]
fn screen_get_text_returns_string_from_part_of_screen() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.write(b"abcd").unwrap();
    vterm.flush().unwrap();

    assert_eq!(vterm.screen_get_text(&Rect::new(Pos::new(1, 0), Size::new(1, 2))).unwrap(),
               "b\nd");
}

#[test]
#[should_panic]
fn screen_get_text_panics_if_rect_is_out_of_bounds() {
    let vterm: VTerm = VTerm::new(&Size {
        height: 1,
        width: 1,
    });
    vterm.screen_get_text(&Rect::new(Pos::new(1, 0), Size::new(1, 2))).unwrap();
}

#[test]
fn screen_get_cells_in_rect_returns_cells() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.write(b"abcd").unwrap();
    vterm.flush().unwrap();

    let cells = vterm.screen_get_cells_in_rect(&Rect::new(Pos::new(0, 0), Size::new(2,2)));
    assert_eq!(cells.iter().map(|c| c.chars[0]).collect::<Vec<u8>>(), b"abcd");
}

#[test]
#[should_panic]
fn screen_get_cells_in_rect_panics_if_out_of_bounds() {
    let vterm: VTerm = VTerm::new(&Size {
        height: 1,
        width: 1,
    });
    vterm.screen_get_cells_in_rect(&Rect::new(Pos::new(1, 0), Size::new(2,2)));
}
