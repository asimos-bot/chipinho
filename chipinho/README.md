## Chipinho

CHIP8 Emulator. Available as a `no_std` lib which makes no assumptions over the system.
The `examples/` folder show many environments that this library can be used.

### Examples

Run examples with `cargo run -p <example> <filename>`:

- `<filename>` - needs to be the path to a valid chip8 program
  - you can use the files at `test_files/`

* `cargo run -p sdl <filename>` - will open a window in your desktop
* `make serve-wasm` - will open a page in your browser

Mind that flickering is actually historically accurate for chip8 programs!

### Error representation using u32

Some functions return `u32` to return a possible error.
* If the first bit is positive, this is an error
* All the other bits are information
  * the lower bits of the first `u16` is used to indicate the type of error (starting from 1)
  * the second `u16` contain error data (if there is any)

The `match` statements at [`error.rs`](./chipinho/src/error.rs) should be clear on these mappings
