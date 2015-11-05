#![feature(libc)]

extern crate rustc_serialize;
extern crate docopt;
extern crate libvterm_sys;
extern crate libc;

use libvterm_sys::*;
use std::io::prelude::*;

enum Format { Plain, Sgr }

struct Context {
    cols_count: usize,
    rows_count: usize,
    format: Format,
}

fn color_to_index(state: &State, target: &Color) -> isize {
    for i in 0..256 {
        let color = state.get_palette_color(i);
        if color.red == target.red && color.green == target.green && color.blue == target.blue {
            return i as isize
        }
    }
    -1
}

fn dump_cell(state: &State, cell: &ScreenCell, prev_cell: &ScreenCell, context: &Context) {
    match context.format {
        Format::Plain => {},
        Format::Sgr => {
            let mut sgrs: Vec<isize> = vec!();

            if !prev_cell.attrs.bold && cell.attrs.bold {
                sgrs.push(1);
            }

            if prev_cell.attrs.bold && !cell.attrs.bold {
                sgrs.push(22);
            }

            if prev_cell.attrs.underline == 0 && cell.attrs.underline != 0 {
                sgrs.push(4);
            }
            if prev_cell.attrs.underline != 0 && cell.attrs.underline == 0 {
                sgrs.push(24);
            }

            if !prev_cell.attrs.italic && cell.attrs.italic {
                sgrs.push(3);
            }
            if prev_cell.attrs.italic && !cell.attrs.italic {
                sgrs.push(23);
            }

            if !prev_cell.attrs.blink && cell.attrs.blink {
                sgrs.push(5);
            }
            if prev_cell.attrs.blink && !cell.attrs.blink {
                sgrs.push(25);
            }

            if !prev_cell.attrs.reverse && cell.attrs.reverse {
                sgrs.push(7);
            }
            if prev_cell.attrs.reverse && !cell.attrs.reverse {
                sgrs.push(27);
            }

            if !prev_cell.attrs.strike && cell.attrs.strike {
                sgrs.push(9);
            }
            if prev_cell.attrs.strike && !cell.attrs.strike {
                sgrs.push(29);
            }

            if prev_cell.attrs.font == 0 && cell.attrs.font != 0 {
                sgrs.push(10 + cell.attrs.font as isize);
            }
            if prev_cell.attrs.font != 0 && cell.attrs.font == 0 {
                sgrs.push(10);
            }

            if prev_cell.fg.red   != cell.fg.red   ||
               prev_cell.fg.green != cell.fg.green ||
               prev_cell.fg.blue  != cell.fg.blue {
                let index = color_to_index(state, &cell.fg);
                if index == -1 {
                    sgrs.push(39);
                }
                else if index < 8 {
                    sgrs.push(30 + index);
                }
                else if index < 16 {
                    sgrs.push(90 + (index - 8));
                }
                else {
                    sgrs.push(38);
                    sgrs.push(5 | (1<<31));
                    sgrs.push(index | (1<<31));
                }
            }

            if prev_cell.bg.red   != cell.bg.red   ||
               prev_cell.bg.green != cell.bg.green ||
               prev_cell.bg.blue  != cell.bg.blue {
                let index = color_to_index(state, &cell.bg);
                if index == -1 {
                    sgrs.push(49);
                }
                else if index < 8 {
                    sgrs.push(40 + index);
                }
                else if index < 16 {
                    sgrs.push(100 + (index - 8));
                }
                else {
                    sgrs.push(48);
                    sgrs.push(5 | (1<<31));
                    sgrs.push(index | (1<<31));
                }
            }

            if sgrs.len() != 0 {
                print!("\x1b[");
                for (i, val) in sgrs.iter().enumerate() {
                    let bare_val = val & !(1<<31);
                    if i == 0 {
                        print!("{}", bare_val);
                    }
                    else if val & (1<<31) != 0 {
                        print!(":{}", bare_val);
                    }
                    else {
                        print!(";{}", bare_val);
                    }
                }
                print!("m");
            }
        }
    }

    std::io::stdout().write_all(&cell.chars_as_utf8_bytes())
                     .ok()
                     .expect("failed to write");
}

fn dump_eol(prev_cell: &ScreenCell, context: &Context) {
    match context.format {
        Format::Plain => {},
        Format::Sgr => {
            if prev_cell.attrs.bold || prev_cell.attrs.underline != 0|| prev_cell.attrs.italic ||
               prev_cell.attrs.blink || prev_cell.attrs.reverse || prev_cell.attrs.strike ||
               prev_cell.attrs.font != 0 {
                print!("\x1b[m");
            }
        }
    }

    print!("\n");
}

fn dump_row(row: usize, vt: &VTerm, context: &Context) {
    let mut prev_cell: ScreenCell = Default::default();
    let (fg, bg) = vt.get_state().get_default_colors();
    prev_cell.fg = fg;
    prev_cell.bg = bg;

    let vts = vt.get_screen();

    let mut pos = Pos { row: row, col: 0 };
    while pos.col < context.cols_count {
        let cell = vts.get_cell(&pos);

        dump_cell(&vt.get_state(), &cell, &prev_cell, context);

        pos.col += cell.width as usize;
        prev_cell = cell;
    }

    dump_eol(&prev_cell, context);
}

const USAGE: &'static str = "
unterm

Usage:
unterm [-c <cols>] [-l <lines>] [-f <format>] <file>

Options:
-c <cols>      number of columns to display
-l <lines>     number of lines in vterm before scrolling
-f <format>    plain or sgr
";

#[derive(Debug, RustcDecodable)]
struct Args {
    flag_c:    usize,
    flag_l:    usize,
    flag_f:    String,
    arg_file:  String,
}

fn main() {
    let mut args: Args = docopt::Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    args.flag_l = if args.flag_l       != 0 { args.flag_l } else { 25 };
    args.flag_c = if args.flag_c       != 0 { args.flag_c } else { 80 };
    args.flag_f = if args.flag_f.len() != 0 { args.flag_f } else { "sgr".to_string() };

    let mut context = Context {
        rows_count:  args.flag_l,
        cols_count:  args.flag_c,
        format:      if args.flag_f == "sgr" { Format::Sgr } else { Format::Plain },
    };

    let mut vt = VTerm::new(context.rows_count, context.cols_count);

    vt.set_utf8(true);

    let rx = vt.receive_screen_events();

    let mut vts: Screen = vt.get_screen();
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

    while let Ok(event) = rx.try_recv() {
        match event {
            ScreenEvent::Resize{rows, cols} =>  {
                context.rows_count = rows;
                context.cols_count = cols;
            },
            ScreenEvent::SbPushLine{cells} => {
                let (fg, bg) = vt.get_state().get_default_colors();
                let mut prev_cell: ScreenCell = Default::default();
                prev_cell.fg = fg;
                prev_cell.bg = bg;

                for cell in cells {
                    dump_cell(&vt.get_state(), &cell, &prev_cell, &context);
                    prev_cell = cell
                }

                dump_eol(&prev_cell, &context);
            },
            _ => {},
        }
    }

    for row in 0..context.rows_count {
        dump_row(row, &vt, &context);
    }
}
