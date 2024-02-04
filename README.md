# INF568 Assignment 4 - x25519

Author: [Clément CHAPOT](mailto:clement.chapot@polytechnique.edu) <br>
Description: implementation of x25519 (see: [RFC 7748](https://datatracker.ietf.org/doc/html/rfc8439)) as part of INF568 course at École polytechnique

## Building

Build the project using `make`.

This calls `cargo build --release` and copies binary `x25519` from `target/release/` into the project root.

## Usage

Run using `./x25519`.

`./x25519` can either be used to derive a public key from a private key (1 argument), or to derive a shared secret from a private key and a public key (two arguments)

For more precise usage information, use `--help` on the relevant binary.

## Testing

`cargo test` checks if the binary and the intermediate functions produce the right output, by using the tests from the RFC.

## Project structure

The core of the project can be found in `src/lib/`.

`src/lib/montgomery.rs` provides functions to perform arithmetic on elliptic curves, which are used in `src/lib/x25519.rs` to implement Diffie-Hellman on the 25519 curve.

The file `src/bin/x25519.rs` is here to produce the binary, so it only contains a main function, which calls functions from `src/lib/` directly.
