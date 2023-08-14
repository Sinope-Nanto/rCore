
const MAX_APP_NUM: usize = 16;

struct AppManager{
    num_app : usize,
    current_app : usize;
    start_addr : [usize; MAX_APP_NUM + 1]
}