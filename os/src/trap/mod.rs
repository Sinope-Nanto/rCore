mod context;

use crate::batch::execute_next_app;
use crate::syscall::syscall;
use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Trap},
    stval, stvec,
};


use crate::log;
use crate::klog;
use crate::error;
use crate::warning;
use crate::info;
use crate::debug;
use crate::trace;
use crate::LOG_FILTER;

global_asm!(include_str!("trap.S"));

/// initialize CSR `stvec` as the entry of `__alltraps`
pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}


#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) |
        Trap::Exception(Exception::StorePageFault) => {
            log!(klog::LOG_LEVEL_ERROR, "[kernel] PageFault in application, kernel killed it.");
            execute_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            log!(klog::LOG_LEVEL_ERROR, "[kernel] IllegalInstruction in application, kernel killed it.");
            execute_next_app();
        }
        _ => {
            panic!("Unsupported trap {:?}, stval = {:#x}!", scause.cause(), stval);
        }
    }
    cx
}

pub use context::TrapContext;