all: build copy

build:
	cargo build --release

copy:
	cp target/release/x25519 x25519

clean:
	cargo clean
	rm x25519
