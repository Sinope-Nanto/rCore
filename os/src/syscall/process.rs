//! App management syscalls
use crate::batch::execute_next_app;
use crate::println;
/// task exits and submit an exit code
pub fn sys_exit(exit_code: i32) -> ! {
    println!("[kernel] Application exited with code {}", exit_code);
    execute_next_app()
}