# Lab 1 Report: System Calls and Basic Time Support

## Overview

This report follows the experiment layout used from Chapter 3. The main goal of
this lab is to understand the user-to-kernel system call path and complete the
basic kernel services needed by the user tests.

The implementation work focuses on:

- entering the kernel from user space through `ecall`;
- dispatching system calls in the kernel;
- returning time information to user space;
- maintaining simple per-task system call statistics.

When moving work between chapter branches, I used the guide's suggestion in
principle: previous chapter commits can be migrated into the current branch with
the `git cherry-pick` family of commands. This keeps each chapter branch based
on the earlier lab work without manually rewriting the same patches.

## System Call Path

User programs do not call kernel functions directly. They call wrapper
functions in the user library, which finally execute `ecall`. The trap handler
enters the kernel, reads the syscall id and arguments, dispatches to the
corresponding kernel implementation, and then writes the return value back into
the saved trap context.

This path is important because all user pointers passed into syscalls are user
virtual addresses. The kernel must translate them through the current task's
page table before reading or writing user memory.

## Time System Call

The `get_time` implementation obtains the current timer value and writes a
`TimeVal` structure back to user memory. The user buffer may cross a page
boundary, so the kernel uses translated byte buffers and copies the structure
across all returned slices.

This avoids directly dereferencing a user virtual address in kernel space.

## Trace Support

The trace-related work records how many times each system call is used by the
current task. The syscall counter is updated in the common syscall dispatch path
before the concrete syscall handler is executed.

For trace read/write operations, only one byte is accessed because the user API
passes a `*const u8` or `*mut u8`. The kernel checks the translated page table
entry and validates permissions before accessing the byte.

## Testing

The relevant Chapter 3 tests include yield, sleep, time, and trace programs.
They verify that:

- user programs can enter the kernel and return correctly;
- timer-based time values are visible from user space;
- syscall counting is updated correctly;
- invalid user memory accesses are rejected instead of being blindly
  dereferenced.

