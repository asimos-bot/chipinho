#![no_std]
extern crate alloc;

use core::alloc::{GlobalAlloc, Layout};
use core::panic::PanicInfo;
use core::arch;
use core::cell::UnsafeCell;

#[cfg(target_arch="wasm32")]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    core::arch::wasm32::unreachable()
}

#[cfg(not(target_arch="wasm32"))]
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    arch::wasm32::unreachable()
}

#[cfg(target_arch="wasm32")]
pub mod allocation;
pub mod emulator;
pub mod instruction;
pub mod prelude;
pub mod error;
pub mod font;
pub mod constants;
