extern crate libc;
extern crate regex;
extern crate term;
extern crate vterm_sys;

use libc::{c_int, c_void};
use std::sync::mpsc::Sender;
use std::io::prelude::*;
use vterm_sys::*;
use term::terminfo::{parm, TermInfo};

#[test]
fn state_can_generate_put_glyph_events() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.state_receive_events(&StateCallbacksConfig::all());
    vterm.write(b"a");

    let rx = vterm.state_event_rx.take().unwrap();

    let mut found_it = false;
    while let Ok(e) = rx.try_recv() {
        match e {
            StateEvent::PutGlyph{glyph_info, pos} => {
                found_it = true;

                assert_eq!(glyph_info.chars[0], b'a');
                assert_eq!(pos.x, 0);
                assert_eq!(pos.y, 0);

                break;
            }
            _ => {}
        }
    }

    assert!(found_it);
}

struct CapBuilder<'a, 'b> {
    terminfo: &'a term::terminfo::TermInfo,
    cap: Option<String>,
    variables: Option<&'b mut term::terminfo::parm::Variables>,
    params: Vec<term::terminfo::parm::Param>,
}

// Use like:
//
// let bytes: Vec<u8> = CapBuilder::new(&terminfo)
//     .cap("cup")
//     .num_param(13)
//     .num_param(12)
//     .build()
//
impl<'a, 'b> CapBuilder<'a, 'b> {
    pub fn new(terminfo: &'a term::terminfo::TermInfo) -> CapBuilder<'a, 'b> {
        CapBuilder {
            terminfo: terminfo,
            cap: None,
            variables: None,
            params: vec![],
        }
    }

    pub fn cap<S>(mut self, cap: S) -> CapBuilder<'a, 'b>
        where S: Into<String>
    {
        self.cap = Some(cap.into());
        self
    }

    pub fn variables(mut self,
                     variables: &'b mut term::terminfo::parm::Variables)
                     -> CapBuilder<'a, 'b> {
        self.variables = Some(variables);
        self
    }

    pub fn number_param(mut self, val: i32) -> CapBuilder<'a, 'b> {
        self.params.push(term::terminfo::parm::Param::Number(val));
        self
    }

    pub fn word_param<S>(mut self, val: S) -> CapBuilder<'a, 'b>
        where S: Into<String>
    {
        self.params.push(term::terminfo::parm::Param::Words(val.into()));
        self
    }

    pub fn build(self) -> Result<Vec<u8>, String> {
        if self.variables.is_some() {
            self.build_with_variables()
        } else {
            self.build_without_variables()
        }
    }

    fn build_with_variables(self) -> Result<Vec<u8>, String> {
        let variables = try! { self.variables.ok_or("oops, expected variables to be defined") };
        let cap = try! { self.cap.ok_or("invalid configuration: cap not provided") };
        let cmd = try! { self.terminfo.strings.get(&cap).ok_or("cap doesn't exist") };

        term::terminfo::parm::expand(&cmd, self.params.as_slice(), variables)
            .or_else(|e| Err(format!("error expanding: {}", e)))
    }

    fn build_without_variables(self) -> Result<Vec<u8>, String> {
        let mut variables = term::terminfo::parm::Variables::new();
        let cap = try! { self.cap.ok_or("invalid configuration: cap not provided") };
        let cmd = try! { self.terminfo.strings.get(&cap).ok_or("cap doesn't exist") };
        term::terminfo::parm::expand(&cmd, self.params.as_slice(), &mut variables)
            .or_else(|e| Err(format!("error expanding: {}", e)))
    }
}

#[test]
fn state_can_generate_move_cursor_events() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.state_receive_events(&StateCallbacksConfig::all());

    let terminfo = term::terminfo::TermInfo::from_name("xterm").unwrap();
    vterm.write(&CapBuilder::new(&terminfo)
                     .cap("cup")
                     .number_param(0)
                     .number_param(1)
                     .build()
                     .unwrap());

    let rx = vterm.state_event_rx.take().unwrap();

    let mut found_it = false;
    while let Ok(e) = rx.try_recv() {
        match e {
            StateEvent::MoveCursor{new, old, is_visible} => {
                found_it = true;

                assert_eq!(new, Pos { x: 1, y: 0 });
                assert_eq!(old, Pos { x: 0, y: 0 });
                assert_eq!(is_visible, true);

                break;
            }
            _ => {}
        }
    }

    assert!(found_it);
}

#[test]
fn state_can_generate_scroll_rect_events() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 2,
        width: 2,
    });
    vterm.state_receive_events(&StateCallbacksConfig::all());
    let terminfo = term::terminfo::TermInfo::from_name("xterm").unwrap();
    vterm.write(&CapBuilder::new(&terminfo)
                     .cap("ri")
                     .build()
                     .unwrap());

    let rx = vterm.state_event_rx.take().unwrap();

    let mut found_it = false;
    while let Ok(e) = rx.try_recv() {
        match e {
            StateEvent::ScrollRect{rect, downward, rightward} => {
                found_it = true;

                assert_eq!(rect, Rect::new(Pos::new(0, 0), Size::new(2, 2)));
                assert_eq!(downward, -1);
                assert_eq!(rightward, 0);

                break;
            }
            _ => {}
        }
    }

    assert!(found_it);
}

#[test]
fn state_can_generate_scroll_rect_events_for_part_of_screen() {
    let mut vterm: VTerm = VTerm::new(&Size {
        height: 5,
        width: 2,
    });
    vterm.state_receive_events(&StateCallbacksConfig::all());
    let terminfo = term::terminfo::TermInfo::from_name("xterm").unwrap();

    vterm.write(&CapBuilder::new(&terminfo)
                     .cap("csr")
                     .number_param(3)
                     .number_param(5)
                     .build()
                     .unwrap());

    vterm.write(&CapBuilder::new(&terminfo)
                     .cap("cup")
                     .number_param(3)
                     .number_param(0)
                     .build()
                     .unwrap());

    vterm.write(&CapBuilder::new(&terminfo)
                     .cap("ri")
                     .build()
                     .unwrap());

    let rx = vterm.state_event_rx.take().unwrap();

    let mut found_it = false;
    while let Ok(e) = rx.try_recv() {
        match e {
            StateEvent::ScrollRect{rect, downward, rightward} => {
                found_it = true;

                assert_eq!(rect, Rect::new(Pos::new(0, 3), Size::new(2, 2)));
                assert_eq!(downward, -1);
                assert_eq!(rightward, 0);

                break;
            }
            _ => {}
        }
    }

    assert!(found_it);
}
