#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]   //测试的入口函数

use core::panic::PanicInfo;
//use crate::{serial_print, println, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    //blog_os::test_panic_handler(info)
    loop {}
}

/*#[test_case]
fn test_println() {
    serial_println!("test_println...");
    println!("test_println output");
    serial_println!("[ok]");
} */