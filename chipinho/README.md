## Chipinho

CHIP8 Emulator. Available as a `no_std` lib which makes no assumptions over the system.
The `examples/` folder show many environments that this library can be used.

What needs to be provided to the lib:

- todo

### Examples

Run examples with `cargo run -p <example> <filename>`:

- `<filename>` - needs to be the path to a valid chip8 program
  - you can use the files at `test_files/`

* `cargo run -p wasm <filename>` - will open a page in your browser
* `cargo run -p sdl <filename>` - will open a window in your desktop
