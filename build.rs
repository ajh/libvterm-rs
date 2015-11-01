extern crate gcc;
extern crate glob;

use glob::glob;
use std::env;

fn main() {
    let vendor_path = env::current_dir().unwrap().join("vendor/libvterm");

    let mut config = gcc::Config::new();
    for file in glob(vendor_path.join("src/*.c").to_str().unwrap()).unwrap() {
        config.file(file.unwrap());
    }
    config.include(vendor_path.join("include").to_str().unwrap());
    config.include(vendor_path.join("src").to_str().unwrap());
    config.compile("libtsm.a");
}
