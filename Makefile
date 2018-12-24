all: target/release/yeast

target/release/yeast:
	cargo build --release

install: target/release/yeast
	cargo install --path .

clean:
	cargo clean

test:
	# Just a text file
	cargo run tests/lorem_ipsum.in | \
		  cmp tests/lorem_ipsum.out
	# Mix several interpreter
	cargo run tests/mixed_salsa.in | \
		  cmp tests/mixed_salsa.out
	# Play with cat behaviors
	cargo run tests/cats_and_dogs.in | \
		  cmp tests/cats_and_dogs.out
	# Use multiline command
	cargo run tests/multiline_cmd.in | \
		  cmp tests/multiline_cmd.out
	# Threading computation
	cargo run tests/123_go.in | \
		  cmp tests/123_go.out
	# Call @gcc kombucha aliase
	#cargo run tests/brewed_kombucha.in | \
	#	  cmp tests/brewed_kombucha.out
	# Use $0, $1, $2, etc... syntax
	cargo run tests/few_args.in foo bar | \
		  cmp tests/few_args.out
	# Use $* for catching all args
	cargo run tests/all_args.in foo bar | \
		  cmp tests/all_args.out

.PHONY: all target/release/yeast install test
