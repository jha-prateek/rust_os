#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::println;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    rust_os::test_panic_handler(_info);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello from Rust OS Kernel v{}", 1.0);

    rust_os::init();

    // Invoking Page Fault interrupt
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

    // Invoking Breakpoint interrupt
    // x86_64::instructions::interrupts::int3();

    // Invoking StackOverflow
    fn stackoverflow() {
        stackoverflow();
    }

    stackoverflow();

    #[cfg(test)]
    test_main();

    println!("Still up!!");

    loop {}
}
