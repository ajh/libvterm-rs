#![feature(libc)]

extern crate rustc_serialize;
extern crate docopt;
extern crate libvterm_sys;
extern crate libc;

use libvterm_sys::*;
use std::io::prelude::*;
use std::slice;

// port libvterms unterm example to rust

//#include <stdio.h>
//#include <string.h>

//#include <errno.h>
//#include <fcntl.h>
//#include <getopt.h>
//#include <unistd.h>

//#include "vterm.h"

//#include "../src/utf8.h" // fill_utf8

//#define streq(a,b) (!strcmp(a,b))

//static VTerm *vt;
//static VTermScreen *vts;

//static int cols;
//static int rows;

//static enum {
  //FORMAT_PLAIN,
  //FORMAT_SGR,
//} format = FORMAT_PLAIN;

enum Format {
    Plain,
    Sgr,
}

struct Context {
    cols_count: usize,
    rows_count: usize,
    format: Format,
}

//static int col2index(VTermColor target)
//{
  //for(int index = 0; index < 256; index++) {
    //VTermColor col;
    //vterm_state_get_palette_color(NULL, index, &col);
    //if(col.red == target.red && col.green == target.green && col.blue == target.blue)
      //return index;
  //}
  //return -1;
//}

fn dump_cell(cell: &Cell, prev_cell: &Cell, context: &Context) {
    match context.format {
        Format::Plain => {},
        Format::Sgr => {
          //{
            //// If all 7 attributes change, that means 7 SGRs max
            //// Each colour could consume up to 3
            //int sgr[7 + 2*3]; int sgri = 0;

            //if(!prevcell->attrs.bold && cell->attrs.bold)
              //sgr[sgri++] = 1;
            //if(prevcell->attrs.bold && !cell->attrs.bold)
              //sgr[sgri++] = 22;

            //if(!prevcell->attrs.underline && cell->attrs.underline)
              //sgr[sgri++] = 4;
            //if(prevcell->attrs.underline && !cell->attrs.underline)
              //sgr[sgri++] = 24;

            //if(!prevcell->attrs.italic && cell->attrs.italic)
              //sgr[sgri++] = 3;
            //if(prevcell->attrs.italic && !cell->attrs.italic)
              //sgr[sgri++] = 23;

            //if(!prevcell->attrs.blink && cell->attrs.blink)
              //sgr[sgri++] = 5;
            //if(prevcell->attrs.blink && !cell->attrs.blink)
              //sgr[sgri++] = 25;

            //if(!prevcell->attrs.reverse && cell->attrs.reverse)
              //sgr[sgri++] = 7;
            //if(prevcell->attrs.reverse && !cell->attrs.reverse)
              //sgr[sgri++] = 27;

            //if(!prevcell->attrs.strike && cell->attrs.strike)
              //sgr[sgri++] = 9;
            //if(prevcell->attrs.strike && !cell->attrs.strike)
              //sgr[sgri++] = 29;

            //if(!prevcell->attrs.font && cell->attrs.font)
              //sgr[sgri++] = 10 + cell->attrs.font;
            //if(prevcell->attrs.font && !cell->attrs.font)
              //sgr[sgri++] = 10;

            //if(prevcell->fg.red   != cell->fg.red   ||
                //prevcell->fg.green != cell->fg.green ||
                //prevcell->fg.blue  != cell->fg.blue) {
              //int index = col2index(cell->fg);
              //if(index == -1)
                //sgr[sgri++] = 39;
              //else if(index < 8)
                //sgr[sgri++] = 30 + index;
              //else if(index < 16)
                //sgr[sgri++] = 90 + (index - 8);
              //else {
                //sgr[sgri++] = 38;
                //sgr[sgri++] = 5 | (1<<31);
                //sgr[sgri++] = index | (1<<31);
              //}
            //}

            //if(prevcell->bg.red   != cell->bg.red   ||
                //prevcell->bg.green != cell->bg.green ||
                //prevcell->bg.blue  != cell->bg.blue) {
              //int index = col2index(cell->bg);
              //if(index == -1)
                //sgr[sgri++] = 49;
              //else if(index < 8)
                //sgr[sgri++] = 40 + index;
              //else if(index < 16)
                //sgr[sgri++] = 100 + (index - 8);
              //else {
                //sgr[sgri++] = 48;
                //sgr[sgri++] = 5 | (1<<31);
                //sgr[sgri++] = index | (1<<31);
              //}
            //}

            //if(!sgri)
              //break;

            //printf("\e[");
            //for(int i = 0; i < sgri; i++)
              //printf(!i               ? "%d" :
                  //sgr[i] & (1<<31) ? ":%d" :
                  //";%d",
                  //sgr[i] & ~(1<<31));
            //printf("m");
          //}
          //break;
        }
    }

    std::io::stdout().write_all(&cell.get_chars_utf8_encoded_as_bytes());
}

fn dump_eol(prev_cell: &Cell, context: &Context) {
    match context.format {
        Format::Plain => {},
        Format::Sgr => {
            //if(prevcell->attrs.bold || prevcell->attrs.underline || prevcell->attrs.italic ||
                //prevcell->attrs.blink || prevcell->attrs.reverse || prevcell->attrs.strike ||
                //prevcell->attrs.font)
            print!("\x1b[m");
        }
    }

    print!("\n");
}

fn dump_row(row: usize, vt: &VTerm, context: &Context) {
    let mut pos = Pos { row: row, col: 0 };
    let mut prev_cell = Cell::new();
    let (fg, bg) = vt.get_state().get_default_colors();
    //prev_cell.set_fg(fg);
    //prev_cell.set_bg(bg);
    let vts = vt.get_screen();

    while pos.col < context.cols_count {
        let cell = vts.get_cell(&pos);

        dump_cell(&cell, &prev_cell, context);

        pos.col += cell.get_width();
        prev_cell = cell;
    }

    dump_eol(&prev_cell, context);
}

// TODO: figure out how to write this at a higher level
extern fn screen_sb_pushline(cols: libc::c_int, cells: *mut ffi::VTermScreenCell, context: *mut Context) -> libc::c_int {
    let mut cells = cells; // not sure why I must do this
    let context: &Context = unsafe { & *context };
    let mut prev_cell = Cell::new();
    //let (fg, bg) = vt.get_state().get_default_colors();
    //prev_cell.set_fg(fg);
    //prev_cell.set_bg(bg);

    for i in 0..cols {
        // This assumes I am in charge of free'ing the passed in cells. I should confirm that!
        let cell = Cell::from_ptr(unsafe { &mut *cells });
        dump_cell(&cell, &prev_cell, context);
        unsafe { cells = cells.offset(1) };
        prev_cell = cell;
    }

    dump_eol(&prev_cell, context);

    1
}

extern fn screen_resize(new_rows: libc::c_int, new_cols: libc::c_int, context: *mut Context) -> libc::c_int {
    unsafe {
        (*context).rows_count = new_rows as usize;
        (*context).cols_count = new_cols as usize;
    }
    1
}

//static VTermScreenCallbacks cb_screen = {
  //.sb_pushline = &screen_sb_pushline,
  //.resize      = &screen_resize,
//};

const USAGE: &'static str = "
unterm

Usage:
unterm [--cols=<cols>] [--rows=<size>] [--format=<size>] <file>

Options:
--cols=<size>      number of columns to display
--rows=<size>      number of rows to display
--format=<format>  plain or sgr
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_cols:    usize,
    flag_rows:    usize,
    flag_format:  String,
    arg_file:     String,
}

fn main() {
    let mut args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    args.flag_rows   = if args.flag_rows         != 0 { args.flag_rows   } else { 25 };
    args.flag_cols   = if args.flag_cols         != 0 { args.flag_cols   } else { 80 };
    args.flag_format = if args.flag_format.len() != 0 { args.flag_format } else { "sgr".to_string() };

    let context = Context {
        rows_count: args.flag_rows,
        cols_count: args.flag_cols,
        format: if args.flag_format == "sgr" { Format::Sgr } else { Format::Plain },
    };

    let mut vt = VTerm::new(context.rows_count, context.cols_count);

    vt.set_utf8(true);

    let mut vts: Screen = vt.get_screen();
    //vterm_screen_set_callbacks(vts, &cb_screen, NULL);
    vts.reset(true);

    let mut file = std::fs::File::open(args.arg_file).unwrap();
    let mut read_buf = [0 as u8; 1024];
    loop {
        match file.read(&mut read_buf) {
            Ok(0)   => break,
            Ok(num) => { vt.write(&read_buf[0..num]); },
            Err(_)  => panic!("error reading from file")
        }
    }

    for row in 0..args.flag_rows {
        dump_row(row, &vt, &context);
    }
}
