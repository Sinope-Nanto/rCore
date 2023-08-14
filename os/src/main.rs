#![no_std]
#![no_main]
#![feature(panic_info_message)]

mod console;
mod lang_items;
mod sbi;
mod klog;
use klog::*;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext(); // begin addr of text segment
        fn etext(); // end addr of text segment
        fn srodata(); // start addr of Read-Only data segment
        fn erodata(); // end addr of Read-Only data ssegment
        fn sdata(); // start addr of data segment
        fn edata(); // end addr of data segment
        fn sbss(); // start addr of BSS segment
        fn ebss(); // end addr of BSS segment
        fn boot_stack_lower_bound(); // stack lower bound
        fn boot_stack_top(); // stack top
    }
    clear_bss();
    Logger::logger_init();

    println!("[kernel] Hello, world!");
    log!(LOG_LEVEL_INFO,"[kernel] .text [{:#x}, {:#x})",
        stext as usize, etext as usize);
    log!(LOG_LEVEL_INFO,"[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize, erodata as usize);
    log!(LOG_LEVEL_INFO, "[kernel] .data [{:#x}, {:#x})",
        sdata as usize, edata as usize);
    log!(LOG_LEVEL_INFO, "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as usize, boot_stack_lower_bound as usize);
    log!(LOG_LEVEL_INFO, "[kernel] .bss [{:#x}, {:#x})", 
        sbss as usize, ebss as usize);    
    panic!("Shutdown machine!");
}

fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}