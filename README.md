## Chipinho

CHIP8 Emulator. Available as a `no_std` lib which makes no assumptions over the system.

What needs to be provided to the lib:
* A function to `set_pixel` at coordinates X, Y to u8
* A function to `get_pixel` at coordinates X, Y
* A function to `beep`
* `tick` must be called at the desired frequency (60 hz usually)
    * with `keypad` state as argument
* `memory` slice to be used as RAM
* `get_random_u8` - function to generate a random byte
