use crate::sbi::console_put_char;
use core::fmt::{self, Write};

struct Stdout;

impl Write for Stdout{
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            console_put_char(c as usize);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments){
    Stdout.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?))
    }
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?))
    }
}

#[macro_export]
macro_rules! info {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[34m", "[INFO] ", $fmt, "\x1b[0m\n") $(, $($arg)+)?))
    }
}

#[macro_export]
macro_rules! warning {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[93m", "[WARN] ", $fmt, "\x1b[0m\n") $(, $($arg)+)?))
    }
}

#[macro_export]
macro_rules! error {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[31m", "[ERROR] ", $fmt, "\x1b[0m\n") $(, $($arg)+)?))
    }
}

#[macro_export]
macro_rules! debug {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[32m", "[DEBUG] ", $fmt, "\x1b[0m\n") $(, $($arg)+)?))
    }
}

#[macro_export]
macro_rules! trace {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!(concat!("\x1b[90m", "[TRACE] ", $fmt, "\x1b[0m\n") $(, $($arg)+)?))
    }
}