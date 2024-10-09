# lc3-vm

An implementation of lc3 vm in Rust.

To test the VM, you can run the games ported to .obj in examples folder, such as 2048 or roguelike, running:

`cargo run examples/<game.obj>`

where `<game.obj>` is the game to be executed.

### Makefile
There's a makefile to make easier the interaction, the commands are:

- `make test` to execute the tests

- `make clippy` to execute clippy

- `make demo` to run a demo with the 2048 game

- `make game name=<game-name>` to run the game of your preference in the examples folder. Just replace `<game-name>` with the name of the game
