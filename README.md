# libxdc-sys

Rust bindings for libxdc, an Intel-PT decoding library for fuzzing.

libxdc requires capstone v4 to be built, refer to [libxdc readme](https://github.com/nyx-fuzz/libxdc?tab=readme-ov-file#install) to install capstone.
There is no need to install `libxdc` since this crate builds the library and statically links it.

**This is still an experiment to be tested.**
