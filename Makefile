all:
	cargo build --release

install:
	cp target/release/yeast /usr/local/bin/
	cp kombucha /usr/local/bin/

test:
	RUST_BACKTRACE=1 cargo run tests/lorem_ipsum.in > debug.log && cmp debug.log tests/lorem_ipsum.out
	RUST_BACKTRACE=1 cargo run tests/basic_example.in > debug.log && cmp debug.log tests/basic_example.out
	RUST_BACKTRACE=1 cargo run tests/cats_and_dogs.in > debug.log && cmp debug.log tests/cats_and_dogs.out
	RUST_BACKTRACE=1 cargo run tests/multiline.in > debug.log && cmp debug.log tests/multiline.out
	RUST_BACKTRACE=1 cargo run tests/123_go.in > debug.log && cmp debug.log tests/123_go.out
