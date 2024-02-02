# INF568 Assignment 2&3 - chacha20/poly1305 AEAD

Author: [Clément CHAPOT](mailto:clement.chapot@polytechnique.edu) <br>
Description: implementation of x25519 (see: [RFC 7748](https://datatracker.ietf.org/doc/html/rfc8439)) as part of INF568 course at École polytechnique

## Building

Build the project using `make`.

This calls `cargo build --release` and copies binaries from `target/release/` into the project root.

## Running

Run using `./x25519`. For more usage information, use `--help` on the relevant binary.

## Testing

Run `cargo test` to check if the binaries produce the right output, checking `./x25519` against `openssl` and against the tests provided in the RFC.

## Project structure

The core of the project can be found in `src/lib/`. The file `src/bin/x25519.rs` is here to produce binaries, so it only contains a main function, which calls functions from `src/lib/` directly.
