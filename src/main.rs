#![feature(panic_handler)]
#![feature(core_intrinsics)]
#![no_std]
#![no_main]

extern crate bootload_precompiled;

use core::intrinsics;
use core::panic::PanicInfo;

#[panic_handler]
#[no_mangle]
fn panic(_info: &PanicInfo) -> ! {
    unsafe { intrinsics::abort() }
}

#[no_mangle]
pub fn _start() -> ! {
    loop {}
}