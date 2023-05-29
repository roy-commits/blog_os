#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::println;
use core::panic::PanicInfo;

// static HELLO: &[u8] = b"Hello World";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    //    use core::fmt::Write;
    //    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    //    write!(
    //        vga_buffer::WRITER.lock(),
    //        ", some numbers: {}, {}",
    //        42,
    //        1.337
    //    )
    //    .unwrap();
    println!("Hello world {}", "!");

    // call init() and create a breakpiont
    blog_os::init();
    // trigger a page fault
    //    unsafe {
    //        *(0xdeadbeef as *mut u64) = 42;
    //    };
    //
    // x86_64::instructions::interrupts::int3();

    // stack_overflow
    fn stack_overflow() {
        stack_overflow();
    }
    stack_overflow();

    #[cfg(test)]
    test_main();
    // add panic handle
    // panic!("Some panic message");
    println!("blog_os did not crash!");
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// panic handler in test mode
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
