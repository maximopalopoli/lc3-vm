# Execute tests
test:
	cargo test

# Execute Clippy
clippy:
	cargo clippy

# Command to run file 2048.obj
demo:
	cargo run examples/2048.obj

# Command to run the file of a given name game
game:
	cargo run examples/$(name).obj

# Default rule
.PHONY: test clippy
