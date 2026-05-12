# Lab 2 Report

## Goal
Implement the Chapter 4 sys_mmap,sys_munmap syscall and pass all Chapter4 tests.Fix the sys_get_times and sys_trace with virtual memory mechanism.

## implement
Add the mmap and munmap function in the related file located at mm directory.Package them in task/mod.rs with `TASK_MANAGER`


## Test

I used the official checker command:

```bash
cd ci-user
make test CHAPTER=4
```

The Chapter 4 checker reported:

```text
Test passed: 8/8
```