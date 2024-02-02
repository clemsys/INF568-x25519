use clap::{arg, command, value_parser};

fn main() {
    // deal with command line arguments
    let matches = command!()
        .arg(
            arg!([M] "hex-encoded string of 32 bytes representing the key interpreted as an integer")
                .required(true)
                .value_parser(value_parser!(String)),
        )
        .arg(
            arg!([U] "hex-encoded string of 32 bytes representing the key interpreted as an integer\nIf not provided, x25519 performs DH public key generation.\nIf provided, x25519 performs DH key exchange.")
                .required(false)
                .value_parser(value_parser!(String)),
        )
        .get_matches();

    let m = matches.get_one::<String>("M").unwrap();
    println!("M: {m}");
}
