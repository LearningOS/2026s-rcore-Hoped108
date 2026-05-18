use crate::sync::{Condvar, Mutex, MutexBlocking, MutexSpin, Semaphore};
use crate::task::{block_current_and_run_next, current_process, current_task};
use crate::timer::{add_timer, get_time_ms};
use alloc::sync::Arc;
use alloc::vec::Vec;

fn ensure_matrix(matrix: &mut Vec<Vec<usize>>, rows: usize, col: usize) {
    while matrix.len() < rows {
        matrix.push(Vec::new());
    }
    for row in matrix.iter_mut() {
        row.resize(col, 0);
    }
}

fn is_safe(
    available: &Vec<usize>,
    allocation: &Vec<Vec<usize>>,
    need: &Vec<Vec<usize>>,
    active: &Vec<bool>,
) -> bool {
    let mut work = available.clone();
    let mut finish: Vec<bool> = active.iter().map(|x| !*x).collect();

    loop {
        let mut found = false;
        for i in 0..active.len() {
            if finish[i] {
                continue;
            }
            let can_finish = need[i].iter().zip(work.iter()).all(|(n, w)| *n <= *w);
            if can_finish {
                for j in 0..work.len() {
                    work[j] += allocation[i][j];
                }

                finish[i] = true;
                found = true;
            }
        }
        if !found {
            break;
        }
    }

    finish.iter().all(|x| *x)
}

/// sleep syscall
pub fn sys_sleep(ms: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_sleep",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let expire_ms = get_time_ms() + ms;
    let task = current_task().unwrap();
    add_timer(expire_ms, task);
    block_current_and_run_next();
    0
}
/// mutex create syscall
pub fn sys_mutex_create(blocking: bool) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_mutex_create",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let mutex: Option<Arc<dyn Mutex>> = if !blocking {
        Some(Arc::new(MutexSpin::new()))
    } else {
        Some(Arc::new(MutexBlocking::new()))
    };
    let mut process_inner = process.inner_exclusive_access();
    let id = if let Some(id) = process_inner
        .mutex_list
        .iter()
        .enumerate()
        .find(|(_, item)| item.is_none())
        .map(|(id, _)| id)
    {
        process_inner.mutex_list[id] = mutex;
        id
    } else {
        process_inner.mutex_list.push(mutex);
        process_inner.mutex_list.len() - 1
    };
    let thread_count = process_inner.tasks.len();
    let mutex_count = process_inner.mutex_list.len();

    process_inner.mutex_available.resize(mutex_count, 0);
    process_inner.mutex_available[id] = 1;

    ensure_matrix(
        &mut process_inner.mutex_allocation,
        thread_count,
        mutex_count,
    );
    ensure_matrix(&mut process_inner.mutex_need, thread_count, mutex_count);
    id as isize
}
/// mutex lock syscall
pub fn sys_mutex_lock(mutex_id: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_mutex_lock",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let mut process_inner = process.inner_exclusive_access();
    let mutex = Arc::clone(process_inner.mutex_list[mutex_id].as_ref().unwrap());
    let tid = current_task()
        .unwrap()
        .inner_exclusive_access()
        .res
        .as_ref()
        .unwrap()
        .tid;
    let active: Vec<bool> = process_inner.tasks.iter().map(|t| t.is_some()).collect();
    let thread_count = process_inner.tasks.len();
    let mutex_count = process_inner.mutex_list.len();

    ensure_matrix(
        &mut process_inner.mutex_allocation,
        thread_count,
        mutex_count,
    );
    ensure_matrix(&mut process_inner.mutex_need, thread_count, mutex_count);

    if process_inner.dead_lock_enabled {
        process_inner.mutex_need[tid][mutex_id] = 1;
        if !is_safe(
            &process_inner.mutex_available,
            &process_inner.mutex_allocation,
            &process_inner.mutex_need,
            &active,
        ) {
            process_inner.mutex_need[tid][mutex_id] = 0;
            return -0xDEAD;
        }
    }
    drop(process_inner);
    drop(process);
    mutex.lock();
    let process = current_process();
    let mut process_inner = process.inner_exclusive_access();
    if process_inner.dead_lock_enabled {
        process_inner.mutex_need[tid][mutex_id] = 0;
        process_inner.mutex_allocation[tid][mutex_id] = 1;
        process_inner.mutex_available[mutex_id] = 0;
    }
    0
}
/// mutex unlock syscall
pub fn sys_mutex_unlock(mutex_id: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_mutex_unlock",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let mutex = Arc::clone(process_inner.mutex_list[mutex_id].as_ref().unwrap());
    drop(process_inner);
    drop(process);
    mutex.unlock();
    let tid = current_task()
        .unwrap()
        .inner_exclusive_access()
        .res
        .as_ref()
        .unwrap()
        .tid;
    let process = current_process();
    let mut inner = process.inner_exclusive_access();
    if inner.dead_lock_enabled {
        inner.mutex_allocation[tid][mutex_id] = 0;
        inner.mutex_available[mutex_id] = 1;
    }
    0
}
/// semaphore create syscall
pub fn sys_semaphore_create(res_count: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_semaphore_create",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let mut process_inner = process.inner_exclusive_access();
    let id = if let Some(id) = process_inner
        .semaphore_list
        .iter()
        .enumerate()
        .find(|(_, item)| item.is_none())
        .map(|(id, _)| id)
    {
        process_inner.semaphore_list[id] = Some(Arc::new(Semaphore::new(res_count)));
        id
    } else {
        process_inner
            .semaphore_list
            .push(Some(Arc::new(Semaphore::new(res_count))));
        process_inner.semaphore_list.len() - 1
    };
    let thread_count = process_inner.tasks.len();
    let sem_count = process_inner.semaphore_list.len();

    process_inner.sem_available.resize(sem_count, 0);
    process_inner.sem_available[id] = res_count;

    ensure_matrix(&mut process_inner.sem_allocation, thread_count, sem_count);
    ensure_matrix(&mut process_inner.sem_need, thread_count, sem_count);
    id as isize
    //would drop the inner automatically
}
/// semaphore up syscall
pub fn sys_semaphore_up(sem_id: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_semaphore_up",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let sem = Arc::clone(process_inner.semaphore_list[sem_id].as_ref().unwrap());
    drop(process_inner);
    sem.up();
    let mut process_inner = process.inner_exclusive_access();
    let tid = current_task()
        .unwrap()
        .inner_exclusive_access()
        .res
        .as_ref()
        .unwrap()
        .tid;
    if process_inner.dead_lock_enabled {
        process_inner.sem_allocation[tid][sem_id] -= 1;
        process_inner.sem_available[sem_id] += 1;
    }
    drop(process_inner);
    0
}
/// semaphore down syscall
pub fn sys_semaphore_down(sem_id: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_semaphore_down",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let mut process_inner = process.inner_exclusive_access();
    let sem = Arc::clone(process_inner.semaphore_list[sem_id].as_ref().unwrap());
    let tid = current_task()
        .unwrap()
        .inner_exclusive_access()
        .res
        .as_ref()
        .unwrap()
        .tid;
    let active: Vec<bool> = process_inner.tasks.iter().map(|t| t.is_some()).collect();
    let thread_count = process_inner.tasks.len();
    let sem_count = process_inner.semaphore_list.len();

    ensure_matrix(&mut process_inner.sem_allocation, thread_count, sem_count);
    ensure_matrix(&mut process_inner.sem_need, thread_count, sem_count);

    if process_inner.dead_lock_enabled {
        process_inner.sem_need[tid][sem_id] += 1;
        if !is_safe(
            &process_inner.sem_available,
            &process_inner.sem_allocation,
            &process_inner.sem_need,
            &active,
        ) {
            process_inner.sem_need[tid][sem_id] -= 1;
            return -0xDEAD;
        }
    }
    drop(process_inner);
    sem.down();
    let mut process_inner = process.inner_exclusive_access();
    if process_inner.dead_lock_enabled {
        process_inner.sem_need[tid][sem_id] -= 1;
        process_inner.sem_allocation[tid][sem_id] += 1;
        process_inner.sem_available[sem_id] -= 1;
    }
    drop(process_inner);
    0
}
/// condvar create syscall
pub fn sys_condvar_create() -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_condvar_create",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let mut process_inner = process.inner_exclusive_access();
    let id = if let Some(id) = process_inner
        .condvar_list
        .iter()
        .enumerate()
        .find(|(_, item)| item.is_none())
        .map(|(id, _)| id)
    {
        process_inner.condvar_list[id] = Some(Arc::new(Condvar::new()));
        id
    } else {
        process_inner
            .condvar_list
            .push(Some(Arc::new(Condvar::new())));
        process_inner.condvar_list.len() - 1
    };
    id as isize
}
/// condvar signal syscall
pub fn sys_condvar_signal(condvar_id: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_condvar_signal",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let condvar = Arc::clone(process_inner.condvar_list[condvar_id].as_ref().unwrap());
    drop(process_inner);
    condvar.signal();
    0
}
/// condvar wait syscall
pub fn sys_condvar_wait(condvar_id: usize, mutex_id: usize) -> isize {
    trace!(
        "kernel:pid[{}] tid[{}] sys_condvar_wait",
        current_task().unwrap().process.upgrade().unwrap().getpid(),
        current_task()
            .unwrap()
            .inner_exclusive_access()
            .res
            .as_ref()
            .unwrap()
            .tid
    );
    let process = current_process();
    let process_inner = process.inner_exclusive_access();
    let condvar = Arc::clone(process_inner.condvar_list[condvar_id].as_ref().unwrap());
    let mutex = Arc::clone(process_inner.mutex_list[mutex_id].as_ref().unwrap());
    drop(process_inner);
    condvar.wait(mutex);
    0
}
/// enable deadlock detection syscall
///
/// YOUR JOB: Implement deadlock detection, but might not all in this syscall
pub fn sys_enable_deadlock_detect(_enabled: usize) -> isize {
    trace!("kernel: sys_enable_deadlock_detect NOT IMPLEMENTED");
    let process = current_process();
    let mut inner = process.inner_exclusive_access();
    match _enabled {
        1 => {
            inner.dead_lock_enabled = true;
            0
        }
        0 => {
            inner.dead_lock_enabled = false;
            0
        }
        _ => return -1,
    }
}
