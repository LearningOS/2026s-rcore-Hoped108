# Lab 4 Report: Address Space Management

## Overview

This lab is based on the `ch4` work. The main goal is to improve virtual memory
management and support user-level memory operations through system calls.

## Implementation

I implemented memory-related system calls such as `mmap`, `munmap`, and `sbrk`.
The kernel checks whether the user address and length are valid, converts the
requested protection bits into page-table permissions, and then updates the
current task's address space.

For `mmap`, the kernel maps a new virtual memory area only when the target range
is page-aligned and does not conflict with existing mappings. For `munmap`, the
kernel removes an existing mapped area only when the requested range is valid.
For `sbrk`, the kernel grows or shrinks the user heap and returns the previous
program break.

## Testing

I used the Chapter 4 tests:

- `ch4_mmap0` to `ch4_mmap3`
- `ch4_unmap`
- `ch4_unmap2`
- `ch4_trace1`
- `ch4b_sbrk`

These tests check memory mapping, unmapping, invalid address handling,
permission checks, and heap growth or shrink behavior.
