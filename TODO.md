# TODO

* [x] take references to things when its more idiomatic
* [x] implement Write trait
* [ ] use libvterm palette api instead of what I rolled on my own
* [x] replace u16 and i16 with usize
* [x] remove positions on screen cells
* [ ] try out the bitflags crate or whatever instead of my c shim code
* [ ] upgrade libvterm?
* [x] rethink representing cell data as char vs Vec<u8> or [u8] or whatever.
* [ ] add methods to ffi datatypes to convert from that and rust
* [x] use geometry library from crates.io

# geometry libraries

* servo has one: https://github.com/servo/euclid
* this look okay: https://crates.io/crates/geom
* here's a list on crates.io: https://crates.io/keywords/geometry?sort=downloads
