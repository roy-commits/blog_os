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
    println!("blog_os is on starting {}", "...");

    // call init() and create a breakpiont
    blog_os::init();
    // trigger a page fault
    //    unsafe {
    //        *(0xdeadbeef as *mut u64) = 42;
    //    };
    //
    // x86_64::instructions::interrupts::int3();

    // unconditional recursion => stack overflow
    // stack_overflow();

    #[cfg(test)]
    test_main();
    // add panic handle
    // panic!("Some panic message");
    println!("blog_os did not crash!");
    //loop {
    //    // simulate deadlock scenes
    //    use blog_os::print;
    //    for _ in 0..10000 {}
    //    print!("-----");
    //}

    // use hlt loop
    blog_os::hlt_loop();
}

#[allow(dead_code, unconditional_recursion)]
fn stack_overflow() {
    // println!("stack size: {:?}", stack.size);
    stack_overflow();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
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
