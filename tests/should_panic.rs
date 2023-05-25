#![no_std]
#![no_main]
// #![feature(custom_test_frameworks)]
// #![test_runner(test_runner)]
// #![reexport_test_harness_main = "test_main"]

use blog_os::{exit_qemu, serial_println, QemuExitCode};
use core::panic::PanicInfo;

#[no_mangle]
pub extern "C" fn _start() {
    should_panic();
    serial_println!("[test did not panic]");
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
//
//pub fn test_runner(tests: &[&dyn Fn()]) {
//    serial_println!("Running tests: {}", tests.len());
//    for test in tests {
//        test();
//        serial_println!("[test did not panic]");
//        exit_qemu(QemuExitCode::Failed);
//    }
//    exit_qemu(QemuExitCode::Success);
//}
//
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[Ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

// #[test_case]
fn should_panic() {
    serial_println!("should_fail...");
    assert_eq!(0, 1);
}
