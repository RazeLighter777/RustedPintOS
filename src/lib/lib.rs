// disable rust standard library
#![no_std]
// disables Rust runtime init,
#![no_main]

// see https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html
#![feature(lang_items)]

// see https://docs.rust-embedded.org/embedonomicon/smallest-no-std.html
#[lang = "eh_personality"]
extern "C" fn eh_personality() {}

use core::panic::PanicInfo;
use core::sync::atomic;
use core::sync::atomic::Ordering;


#[no_mangle]
/// The name **must be** `_start`, otherwise the compiler throws away all code as unused. 
/// The name can be changed by passing a different entry symbol as linker argument.
fn _start() -> ! {
    loop {
    }
}

#[inline(never)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        atomic::compiler_fence(Ordering::SeqCst);
    }
}


#[no_mangle]
pub extern "C" fn atoi(mut s: *const u8) -> i32 {
    unsafe {
        let mut n = 0;
        while *s != 0 {
            n = n * 10 + (*s - b'0') as i32;
            s = s.offset(1);
        }
        n
    }
    
}