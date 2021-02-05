#![no_std] //不与rust标准库的链接
#![no_main]  //禁用所有Rust层级的入口点
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
//#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

//use crate::println;
use core::panic::PanicInfo;

mod vga_buffer;
mod serial;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("hello world{}", "!");
    #[cfg(test)]
    test_main();
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {}
}

#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    println!("Running {} tests", tests.len());
    for test in tests {
        test();
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    //blog_os::test_panic_handler(_info)
    println!("panic info: {}", _info);
    loop {}
}


#[test_case]
fn trivial_assertion(){
    println!("trivial assertion...");
    assert_eq!(1, 1);
    println!("[ok]");
}


#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    unsafe {  //写入I/O端口可能会导致不可预知的行为
        let mut port = x86_64::instructions::port::Port::new(0xf4);
        port.write(exit_code as u32);  //设置iosize为4字节
    }
}
