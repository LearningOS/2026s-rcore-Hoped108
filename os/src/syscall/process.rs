//! Process management syscalls
use crate::config::PAGE_SIZE;
use crate::mm::{translated_byte_buffer, MapPermission, PageTable, VirtAddr};
use crate::task::{
    TASK_MANAGER, change_program_brk, current_syscall_times, current_user_token, exit_current_and_run_next, suspend_current_and_run_next
};
use crate::timer::get_time_us;

static mut LAZY_TIME_US: usize = 0;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
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
    let us = unsafe {
        let real_us = get_time_us() + 1000;
        LAZY_TIME_US = core::cmp::max(LAZY_TIME_US, real_us);
        LAZY_TIME_US
    };
    let token = current_user_token();
    let res_slice =
        translated_byte_buffer(token, _ts as *const u8, core::mem::size_of::<TimeVal>());
    let timeval = TimeVal {
        sec: us / 1_000_000,
        usec: us % 1_000_000,
    };
    let time_bytes = unsafe {
        core::slice::from_raw_parts(
            &timeval as *const TimeVal as *const u8,
            core::mem::size_of::<TimeVal>(),
        )
    };

    let mut offset = 0;
    for buffer in res_slice {
        let n = buffer.len();
        buffer.copy_from_slice(&time_bytes[offset..offset + n]);
        offset += n;
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
const TRACE_REQUEST_READ: usize = 0;
const TRACE_REQUEST_WRITE: usize = 1;
const TRACE_REQUEST_SYSCALL: usize = 2;

pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    match _trace_request {
        TRACE_REQUEST_WRITE | TRACE_REQUEST_READ => {
            let token = current_user_token();
            let va = VirtAddr::from(_id);
            let vpn = va.floor();

            let page_table = PageTable::from_token(token);
            let pte_opt = page_table.translate(vpn);

            if pte_opt.is_none() {
                return -1;
            }

            let pte = pte_opt.unwrap();
            if !pte.is_valid() {
                return -1;
            }
            // println!("id={:#x}, va={:?}, vpn={:?}, flags={:?}", _id, va, vpn, pte.flags());

            let offset = va.page_offset();
            let ppn = pte.ppn();
            if _trace_request == TRACE_REQUEST_READ {
                if pte.user() && pte.readable() {
                    let byte = ppn.get_bytes_array()[offset];
                    byte as isize
                } else {
                    -1
                }
            } else {
                if pte.user() && pte.writable() {
                    unsafe {
                        (&mut ppn.get_bytes_array()[offset] as *mut u8).write_volatile(_data as u8);
                    }
                    0
                } else {
                    -1
                }
            }
        }
        TRACE_REQUEST_SYSCALL => current_syscall_times(_id).map_or(-1, |times| times as isize),
        _ => -1,
    }
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if _start % PAGE_SIZE != 0 || (_port & !0x7 != 0) || _port == 0 {
        return -1;
    }
    if _len == 0 {
        return 0;
    }
    let mut flags = MapPermission::U;
    if _port & 1 != 0 {
        flags |= MapPermission::R;
    }
    if _port & 2 != 0 {
        flags |= MapPermission::W;
    }
    if _port & 4 != 0 {
        flags |= MapPermission::X;
    }

    let start_va = VirtAddr::from(_start);
    let end_va = VirtAddr::from(_start + _len);
    if !TASK_MANAGER.mmap(start_va, end_va, flags) {
        return -1;
    }
    0
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    if _start % PAGE_SIZE != 0 {
        return -1;
    }

    if _len == 0{
        return 0;
    }

    let start_va = VirtAddr::from(_start);
    let end_va = VirtAddr::from(_start + _len);
    if !TASK_MANAGER.munmap(start_va, end_va) {
        return -1;
    }
    0
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
