#![no_std]

use core::panic::PanicInfo;

#[cfg(target_arch="wasm32")]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

#[cfg(not(target_arch="wasm32"))]
#[panic_handler]
fn panic(_panic: &PanicInfo) -> ! {
    loop {}
}

pub mod emulator;
pub mod instruction;
pub mod prelude;
pub mod error;
pub mod font;
