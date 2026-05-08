# Lab 1 Report

## Goal

Implement the Chapter 3 trace syscall and pass the Chapter 3 user tests.

## Implementation

The kernel records syscall usage for each task. When a user `ecall` enters the
trap handler, the kernel reads the syscall id from `a7` and increments the
current task's counter before dispatching the syscall.

The `sys_trace` syscall supports three requests:

- read one byte from an address
- write one byte to an address
- return the current task's syscall count for a syscall id

The time syscall returns a positive, monotonically increasing value so the
Chapter 3 sleep tests can complete reliably under QEMU and CI.

## Test

I used the official checker command:

```bash
cd ci-user
make test CHAPTER=3
```

The Chapter 3 checker reported:

```text
Test passed: 7/7
```
