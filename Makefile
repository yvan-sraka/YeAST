all:
	cargo build --release

install:
	cp target/release/yeast /usr/local/bin/
