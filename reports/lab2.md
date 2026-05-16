# Lab 2 Report: Address Space Management

## Overview

This lab extends the memory-management part of the kernel. The main goal is to
support user-controlled virtual memory mappings through `mmap` and `munmap`.

The implementation builds on the Chapter 3 syscall path. User programs still
enter the kernel through `ecall`, but the kernel now needs to modify the current
task's address space safely.

## mmap Design

The `mmap` system call maps a user virtual address range with the requested
permissions. The implementation checks:

- the start address must be page-aligned;
- the length of zero is accepted as a no-op;
- permission bits must be valid;
- at least one of read, write, or execute permission must be requested;
- the target virtual pages must not already be mapped.

After validation, the kernel converts the requested permission bits into
`MapPermission` flags and inserts a framed area into the current task's
`MemorySet`.

## munmap Design

The `munmap` system call removes a previously mapped area. It also checks page
alignment and treats zero length as a no-op.

The implementation removes the matching framed area from the task's
`MemorySet`, unmaps the pages from the page table, and releases the tracked
frames through normal ownership drop behavior.

## Page Table Handling

The important distinction is between metadata and real page-table memory.
`MemorySet` and `MapArea` describe the virtual memory layout, while actual page
table pages and mapped data pages are physical frames allocated by the frame
allocator.

When a new area is inserted, the kernel creates page table entries for the
covered virtual pages. When an area is removed, those entries are cleared and
the corresponding frame trackers are dropped.

## Testing

The Chapter 4 memory tests check valid mappings, invalid addresses, overlapping
mappings, permission handling, and unmapping behavior. Passing these tests shows
that the kernel modifies only the current task's address space and rejects
invalid requests cleanly.

