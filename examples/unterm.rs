extern crate rustc_serialize;
extern crate docopt;
extern crate vterm_sys;

use vterm_sys::*;
use std::io::prelude::*;

enum Format {
    Plain,
    Sgr,
}

struct Context {
    cols_count: usize,
    rows_count: usize,
    format: Format,
}

fn dump_cell(vterm: &VTerm, cell: &ScreenCell, prev_cell: &ScreenCell, context: &Context) {
    match context.format {
        Format::Plain => {}
        Format::Sgr => {
            let mut sgrs: Vec<usize> = vec![];

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
                sgrs.push(10 + cell.attrs.font as usize);
            }
            if prev_cell.attrs.font != 0 && cell.attrs.font == 0 {
                sgrs.push(10);
            }

            if prev_cell.fg_rgb.red != cell.fg_rgb.red ||
               prev_cell.fg_rgb.green != cell.fg_rgb.green ||
               prev_cell.fg_rgb.blue != cell.fg_rgb.blue {
                let index = vterm.state_get_palette_color_from_rgb(&cell.fg_rgb);
                if index < 8 {
                    sgrs.push(30 + index as usize);
                } else if index < 16 {
                    sgrs.push(90 + (index as usize - 8));
                } else {
                    sgrs.push(38);
                    sgrs.push(5 | (1 << 31));
                    sgrs.push(index as usize | (1 << 31));
                }
            }

            if prev_cell.bg_rgb.red != cell.bg_rgb.red ||
               prev_cell.bg_rgb.green != cell.bg_rgb.green ||
               prev_cell.bg_rgb.blue != cell.bg_rgb.blue {
                let index = vterm.state_get_palette_color_from_rgb(&cell.bg_rgb);
                if index < 8 {
                    sgrs.push(40 + index as usize);
                } else if index < 16 {
                    sgrs.push(100 + (index as usize - 8));
                } else {
                    sgrs.push(48);
                    sgrs.push(5 | (1 << 31));
                    sgrs.push(index as usize | (1 << 31));
                }
            }

            if sgrs.len() != 0 {
                print!("\x1b[");
                for (i, val) in sgrs.iter().enumerate() {
                    let bare_val = val & !(1 << 31);
                    if i == 0 {
                        print!("{}", bare_val);
                    } else if val & (1 << 31) != 0 {
                        print!(":{}", bare_val);
                    } else {
                        print!(";{}", bare_val);
                    }
                }
                print!("m");
            }
        }
    }

    std::io::stdout()
        .write_all(&cell.chars)
        .ok()
        .expect("failed to write");
}

fn dump_eol(prev_cell: &ScreenCell, context: &Context) {
    match context.format {
        Format::Plain => {}
        Format::Sgr => {
            if prev_cell.attrs.bold || prev_cell.attrs.underline != 0 ||
               prev_cell.attrs.italic || prev_cell.attrs.blink ||
               prev_cell.attrs.reverse || prev_cell.attrs.strike ||
               prev_cell.attrs.font != 0 {
                print!("\x1b[m");
            }
        }
    }

    print!("\n");
}

fn dump_row(row: usize, vt: &VTerm, context: &Context) {
    let mut prev_cell: ScreenCell = Default::default();
    let (fg_rgb, bg_rgb) = vt.state_get_default_colors();
    prev_cell.fg_rgb = fg_rgb;
    prev_cell.bg_rgb = bg_rgb;

    let mut pos = Pos { x: row, y: 0 };
    while pos.x < context.cols_count as usize {
        let cell = vt.screen_get_cell(&pos);

        dump_cell(&vt, &cell, &prev_cell, context);

        pos.x += cell.width as usize;
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
    flag_c: usize,
    flag_l: usize,
    flag_f: String,
    arg_file: String,
}

fn main() {
    let mut args: Args = docopt::Docopt::new(USAGE)
                             .and_then(|d| d.decode())
                             .unwrap_or_else(|e| e.exit());

    args.flag_l = if args.flag_l != 0 {
        args.flag_l
    } else {
        25
    };
    args.flag_c = if args.flag_c != 0 {
        args.flag_c
    } else {
        80
    };
    args.flag_f = if args.flag_f.len() != 0 {
        args.flag_f
    } else {
        "sgr".to_string()
    };

    let mut context = Context {
        rows_count: args.flag_l,
        cols_count: args.flag_c,
        format: if args.flag_f == "sgr" {
            Format::Sgr
        } else {
            Format::Plain
        },
    };

    let mut vt = VTerm::new(&Size {
        height: context.rows_count,
        width: context.cols_count,
    });

    vt.set_utf8(true);

    vt.screen_receive_events(&ScreenCallbacksConfig::all());
    let rx = vt.screen_event_rx.take().unwrap();

    vt.screen_reset(true);

    let mut file = std::fs::File::open(args.arg_file).unwrap();
    let mut read_buf = [0 as u8; 1024];
    loop {
        match file.read(&mut read_buf) {
            Ok(0) => break,
            Ok(num) => {
                vt.write(&read_buf[0..num]).unwrap();
            }
            Err(_) => panic!("error reading from file"),
        }
    }

    while let Ok(event) = rx.try_recv() {
        match event {
            ScreenEvent::Resize{height, width} => {
                context.rows_count = height;
                context.cols_count = width;
            }
            ScreenEvent::SbPushLine{cells} => {
                let (fg_rgb, bg_rgb) = vt.state_get_default_colors();
                let mut prev_cell: ScreenCell = Default::default();
                prev_cell.fg_rgb = fg_rgb;
                prev_cell.bg_rgb = bg_rgb;

                for cell in cells {
                    dump_cell(&vt, &cell, &prev_cell, &context);
                    prev_cell = cell
                }

                dump_eol(&prev_cell, &context);
            }
            _ => {}
        }
    }

    for row in 0..context.rows_count {
        dump_row(row as usize, &vt, &context);
    }
}
