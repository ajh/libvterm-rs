# How to build

    git submodule init
    git submodule update
    (cd vendor/libvterm; make)
    cargo build

The make part in libvterm runs a perl script to transform some files in
libvterm/src/encoding.
