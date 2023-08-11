use core::panic::PanicInfo;

use crate::sbi::shutdown;

use crate::log;
use crate::klog;
use crate::error;
use crate::warning;
use crate::info;
use crate::debug;
use crate::trace;

#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    if let Some(location) = info.location() {
        log!(klog::LOG_LEVEL_ERROR, "Panicked at {}:{} {}",
        location.file(),
        location.line(),
        info.message().unwrap());
    } else {
        log!(klog::LOG_LEVEL_ERROR, "Panicked at {}",
            info.message().unwrap());
    }
    shutdown()
}