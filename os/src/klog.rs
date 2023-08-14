// pub static mut LOG_FILTER: usize = 0;
use crate::println;
extern crate lazy_static;
use lazy_static::lazy_static;


pub const LOG_LEVEL_TRACE: usize = 0;
pub const LOG_LEVEL_DEBUG: usize = 1;
pub const LOG_LEVEL_INFO: usize = 2;
pub const LOG_LEVEL_WARN: usize = 3;
pub const LOG_LEVEL_ERROR: usize = 4;
pub const LOG_LEVEL_NONE: usize = 5;

lazy_static!{
    pub static ref LOG_FILTER : usize = match option_env!("LOG") {
        Some("NONE") => LOG_LEVEL_NONE,
        Some("ERROR") => LOG_LEVEL_ERROR,
        Some("WARN") => LOG_LEVEL_WARN,
        Some("INFO") => LOG_LEVEL_INFO,
        Some("DEBUG") => LOG_LEVEL_DEBUG,
        Some("TRACE") => LOG_LEVEL_TRACE,
        _ => LOG_LEVEL_NONE,
    };
}

pub struct Logger;



impl Logger{
    pub fn logger_init() -> (){
            match *LOG_FILTER {
                LOG_LEVEL_NONE => println!("[kernel] Info output level: None"),
                LOG_LEVEL_ERROR => println!("[kernel] Info output level: Error"),
                LOG_LEVEL_WARN => println!("[kernel] Info output level: Warnning"),
                LOG_LEVEL_INFO => println!("[kernel] Info output level: Info"),
                LOG_LEVEL_DEBUG => println!("[kernel] Info output level: Debug"),
                LOG_LEVEL_TRACE => println!("[kernel] Info output level: Trace"),
                _ => println!("[kernel] Info output level: None"),
        };
    }
}


#[macro_export]
macro_rules! log{
    ($level: expr, $fmt: literal $(, $($arg: tt)+)?) => {
        if $level >= *klog::LOG_FILTER{
            match $level {
                klog::LOG_LEVEL_ERROR => error!($fmt $(, $($arg)+)?),
                klog::LOG_LEVEL_WARN => warning!($fmt $(, $($arg)+)?),
                klog::LOG_LEVEL_INFO => info!($fmt $(, $($arg)+)?),
                klog::LOG_LEVEL_DEBUG => debug!($fmt $(, $($arg)+)?),
                klog::LOG_LEVEL_TRACE => trace!($fmt $(, $($arg)+)?),
                _ => ()
            };
        }
    }
}



