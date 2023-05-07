//! Process management syscalls
use crate::{
    config::MAX_SYSCALL_NUM,
    task::{
        get_task_info,current_user_token,change_program_brk, exit_current_and_run_next, suspend_current_and_run_next, TaskStatus,
    },
    mm::{KERNEL_SPACE,translated_physical_address},
    timer::get_time_us,
    
};

#[repr(C)]
#[derive(Debug)]
///
pub struct TimeVal {
    ///
    pub sec: usize,
    ///
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    pub status: TaskStatus,
    /// The numbers of syscall called by task
   pub  syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
   pub time: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let _us = get_time_us();
    let token1=current_user_token();
    let ts = translated_physical_address(token1,_ts as *const u8 ) as *mut TimeVal;
    unsafe {
         *ts = TimeVal {
             sec: _us / 1_000_000,
            usec: _us % 1_000_000,
        };
     }
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(_ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info NOT IMPLEMENTED YET!");
    let token1=current_user_token();
    let ti =  translated_physical_address(token1,_ti as *const u8 ) as *mut TaskInfo;
    unsafe{
        
            *ti=get_task_info();
    
        


}
    0
}

/// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    let flag= _start%4096;
    let  mut rellen:usize=_len;
    if (_len%4096)!=0
    {
        let num=_len/4096;
        rellen=(num+1)*4096;
    }
    
   // let reallen=(_len%4096)*4096+_len;
    let flag2=_port& 0x7;
    if flag==0 &&flag2>0&&_port<8
    {
        let mut kernel_space = KERNEL_SPACE.exclusive_access();
    let a=kernel_space.mmap(_start,rellen,_port);
    println!("aaaaaaaaaaaaaaaaaaaaaaaaaaa={}",a);
    a
    
    }
    else {
        -1
    }
}

///YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    
    let flag= _start%4096;
    let  mut rellen:usize=_len;
    if (_len%4096)!=0
    {
        let num=_len/4096;
        rellen=(num+1)*4096;
    }
    //let flag2=_port& 0x7;
    if flag==0 
    {
        let mut kernel_space = KERNEL_SPACE.exclusive_access();
        kernel_space.munmap(_start,rellen);
    0
    }
    else {
        -1
    }

}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
