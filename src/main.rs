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
    // let vga_buffer = 0xb8000 as *mut u8;
    //
    // for (i, &byte) in HW_STRING.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }

    // vga_buffer::print_something();

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_string("Hello from Rust OS Kernel\n");
    // write!(vga_buffer::WRITER.lock(), "Some numbers: {} {}\n", 2, 3.0/60.0).unwrap();

    println!("Hello from Rust OS Kernel v{}", 1.0);
    panic!("panicked");

    loop {}
}
