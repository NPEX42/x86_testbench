#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![feature(decl_macro)]
#![test_runner(crate::tests::test_runner)]
#![reexport_test_harness_main = "test_start"]

pub mod tests;
pub mod logger;
pub mod qemu;
pub use bootloader;
use logger::log;

use crate::logger::error;

pub macro boot($func:path) {
    bootloader::entry_point!(boot);
    pub fn boot(_info: &'static bootloader::BootInfo) -> ! {
        let boot_func: fn(&'static bootloader::BootInfo) = $func;
        boot_func(_info);
        loop {};
    }
}


#[cfg_attr(not(all(feature = "ext_panic", test)), panic_handler)]
pub fn on_panic(_info : &core::panic::PanicInfo) -> ! {
    error!("[FAILED]");
    loop {}
}


#[cfg(test)]
#[test_case]
pub fn meta_test() {
    assert_eq!(1,1);
}

