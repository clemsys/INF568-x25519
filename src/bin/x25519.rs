use clap::{arg, command, value_parser};
use std::fmt::Write;
use x25519::lib::x25519::{generate_public_key, generate_shared_secret, Bytes};

fn bytes_from_str(s: &str) -> Bytes {
    assert!(s.len() == 64);
    s.chars()
        .collect::<Vec<char>>()
        .chunks(2)
        .map(|chunk| chunk.iter().collect::<String>())
        .map(|chunk| u8::from_str_radix(&chunk, 16).unwrap())
        .collect::<Vec<u8>>() // is 32 bytes
        .try_into()
        .unwrap()
}

fn string_from_bytes(bytes: &Bytes) -> String {
    bytes.iter().fold(String::new(), |mut output, b| {
        let _ = write!(output, "{b:02x}");
        output
    })
}

fn main() {
    // deal with command line arguments
    let matches = command!()
        .arg(
            arg!([M] "hex-encoded string of 32 bytes, representing the private key, interpreted as an integer")
                .required(true)
                .value_parser(value_parser!(String)),
        )
        .arg(
            arg!([U] "hex-encoded string of 32 bytes, representing the public key of the peer, interpreted as an integer\nIf not provided, x25519 performs DH public key generation.\nIf provided, x25519 performs DH key exchange.")
                .required(false)
                .value_parser(value_parser!(String)),
        )
        .get_matches();

    let m = matches
        .get_one::<String>("M")
        .expect("M is not a valid hex-encoded string of 32 bytes");
    assert!(
        m.len() == 64,
        "M is not a valid hex-encoded string of 32 bytes"
    );
    let m = bytes_from_str(m);

    matches.get_one::<String>("U").map_or_else(
        || {
            // key generation mode
            print!("{}", string_from_bytes(&generate_public_key(&m)));
        },
        |u| {
            // key exchange mode
            assert!(
                u.len() == 64,
                "U is not a valid hex-encoded string of 32 bytes"
            );
            let u = bytes_from_str(u);
            print!("{}", string_from_bytes(&generate_shared_secret(&m, &u)));
        },
    );
}
