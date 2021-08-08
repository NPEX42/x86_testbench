use lazy_static::lazy_static;
use uart_16550::SerialPort;
use spin::Mutex;
use x86_64::instructions::interrupts::without_interrupts;
use core::{fmt::*};

lazy_static! {
    static ref SERIAL : Mutex<SerialPort> = Mutex::new(unsafe {
        SerialPort::new(0x3F8)
    });
}

pub macro debug($($args:tt)*) {
    $crate::logger::_debug(format_args!("\x1b[0;36m[DEBUG] - \x1b[0m"));
    $crate::logger::_debug(format_args!($($args)*));
    $crate::logger::_debug(format_args!("\r\n"));
}

pub macro log($($args:tt)*) {
    $crate::logger::_log(format_args!("\x1b[0;32m[LOG] - \x1b[0m"));
    $crate::logger::_log(format_args!($($args)*));
    $crate::logger::_debug(format_args!("\r\n"));
}

pub macro warn($($args:tt)*) {
    $crate::logger::_warn(format_args!("\x1b[0;33m[WARN] - \x1b[0m"));
    $crate::logger::_warn(format_args!($($args)*));
    $crate::logger::_debug(format_args!("\r\n"));
}


pub macro error($($args:tt)*) {
    $crate::logger::_error(format_args!("\x1b[0;31m[LOG] - \x1b[0m"));
    $crate::logger::_error(format_args!($($args)*));
    $crate::logger::_debug(format_args!("\r\n"));
}

pub macro print($($args:tt)*) {
    $crate::logger::_log(format_args!($($args)*));
    
}

pub macro println($($args:tt)*) {
    $crate::logger::_log(format_args!($($args)*));
    $crate::logger::_log(format_args!("\r\n"));
}

#[doc(hidden)]
pub fn _debug(args : Arguments) {
    without_interrupts(|| {
        SERIAL.lock().write_fmt(args).expect("");
    });
}

#[doc(hidden)]
pub fn _log(args : Arguments) {
    without_interrupts(|| {
        SERIAL.lock().write_fmt(args).expect("");
    });
}

#[doc(hidden)]
pub fn _warn(args : Arguments) {
    without_interrupts(|| {
        SERIAL.lock().write_fmt(args).expect("");
    });
}

#[doc(hidden)]
pub fn _error(args : Arguments) {
    without_interrupts(|| {
        SERIAL.lock().write_fmt(args).expect("");
    });
}
