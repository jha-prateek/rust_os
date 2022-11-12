#![no_std]
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

static HW_STRING: &[u8] = b"Hello from Rust OS Kernel";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello from Rust OS Kernel v{}", 1.0);

    loop {}
}
