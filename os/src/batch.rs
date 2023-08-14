use crate::sbi::shutdown;
use crate::sync::UPSafeCell;
use lazy_static::*;
use core::arch::asm;

const MAX_APP_NUM: usize = 16;

const APP_BASE_ADDRESS: usize = 0x80400000;
const APP_SIZE_LIMIT: usize = 0x20000;
const USER_STACK_SIZE: usize = 4096 * 2;
const KERNEL_STACK_SIZE: usize = 4096 * 2;

#[repr(align(4096))]
struct KernelStack {
    data: [u8; KERNEL_STACK_SIZE],
}

#[repr(align(4096))]
struct UserStack {
    data: [u8; USER_STACK_SIZE],
}

static KERNEL_STACK: KernelStack = KernelStack { data: [0; KERNEL_STACK_SIZE] };
static USER_STACK: UserStack = UserStack { data: [0; USER_STACK_SIZE] };

struct AppManager{
    num_app : usize,
    current_app : usize;
    start_addr : [usize; MAX_APP_NUM + 1]
}

lazy_static! {
    static ref APP_MANAGER: UPSafeCell<AppManager> = unsafe{
        UPSafeCell::new({
            extern "C" {fn _num_app();}  
            let num_app_ptr = _num_app as usize as *const usize;
            let mut start_addr: [usize; MAX_APP_NUM + 1] = [0; MAX_APP_NUM + 1];
            let app_start_raw: &[usize] =  core::slice::from_raw_parts(
                num_app_ptr.add(1), num_app + 1);
            start_addr[..=num_app].copy_from_slice(app_start_raw);
            AppManager{
                num_app,
                current_app:0,
                start_addr,
            }
        })
    };
}

impl AppManager{
    pub fn print_app_info(&self){
        println!("[kernel] num_app = {}", self.num_app);
        for i in 0..self.num_app {
            println!(
                "[kernel] app_{} [{:#x}, {:#x})",
                i,
                self.app_start[i],
                self.app_start[i + 1]
            );
        };
    }

    pub fn get_current_app(&self) -> usize {
        self.current_app
    }

    pub fn move_to_next_app(&mut self) {
        self.current_app += 1;
    }

    unsafe fn load_app(&self, app_id:usize){
        if app_id >= self.num_app{
            panic!("All applications completed!");
        }
        println!("[kernel] Loading app_{}", app_id);
        
        core::slice::from_raw_parts_mut(
            APP_BASE_ADDRESS as *mut u8,
            APP_SIZE_LIMIT).fill(0);
        
        let app_src = from_raw_parts(
            self.start_addr[app_id] as *const u8,
            self.start_addr[app_id + 1] - self.app_start[app_id]
        );

        let app_dst = core::slice::from_raw_parts_mut(
            APP_BASE_ADDRESS as *mut u8,
            app_src.len()
        );
        app_dst.copy_from_slice(app_src);
        asm!("fence.i");// update cache
    }
}

impl UserStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}

impl KernelStack {
    fn get_sp(&self) -> usize {
        self.data.as_ptr() as usize + USER_STACK_SIZE
    }
}