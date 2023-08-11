pub static mut LOG_FILTER: usize = 0;


pub const LOG_LEVEL_TRACE: usize = 0;
pub const LOG_LEVEL_DEBUG: usize = 1;
pub const LOG_LEVEL_INFO: usize = 2;
pub const LOG_LEVEL_WARN: usize = 3;
pub const LOG_LEVEL_ERROR: usize = 4;
pub const LOG_LEVEL_NONE: usize = 5;


pub struct Logger;


impl Logger{
    pub fn logger_init() -> (){
        unsafe{
            LOG_FILTER = match option_env!("LOG") {
                Some("NONE") => LOG_LEVEL_NONE,
                Some("ERROR") => LOG_LEVEL_ERROR,
                Some("WARN") => LOG_LEVEL_WARN,
                Some("INFO") => LOG_LEVEL_INFO,
                Some("DEBUG") => LOG_LEVEL_DEBUG,
                Some("TRACE") => LOG_LEVEL_TRACE,
                _ => LOG_LEVEL_TRACE,
            };
        };
    }
}


#[macro_export]
macro_rules! log{
    ($level: expr, $fmt: literal $(, $($arg: tt)+)?) => {
        let b : bool;
        unsafe {
            b = $level >= klog::LOG_FILTER
        }
        if b {
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



