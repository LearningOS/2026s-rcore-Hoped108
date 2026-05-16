# Lab 6 Report: File System

## Overview

This lab is based on the `ch6` branch. The main goal is to add file-system
support to the kernel and make user programs access files through system calls.

In this chapter, user applications are stored in the file system instead of
being directly embedded into the kernel image. The kernel opens the program file,
reads its ELF content, and then creates a process from it.

## Implementation

The file system is built on top of a block device. The `easy-fs` layer manages
the persistent file-system layout, including the super block, inode bitmap, data
bitmap, disk inodes, and data blocks. The kernel opens the root inode from the
block device and uses it to find, create, link, and unlink files.

In the kernel, `OSInode` wraps an `easy_fs::Inode` and implements the common
`File` trait. Each opened file has its own offset, readable flag, and writable
flag. A process stores opened files in its `fd_table`, so system calls can use a
file descriptor to find the real file object.

The basic file system calls are implemented as follows:

- `open` translates the user path, opens or creates the file, allocates a file
  descriptor, and stores the file object in the current task's `fd_table`.
- `read` and `write` translate the user buffer, check the file permission, and
  call the corresponding `File` trait method.
- `close` removes the file object from the file descriptor table.
- `fstat` gets file metadata from the file object and writes a `Stat` structure
  back to user memory.
- `linkat` creates a new directory entry pointing to the same inode as an
  existing file.
- `unlinkat` removes a directory entry and updates the visible link count.

## Testing

I used the Chapter 6 file-system tests:

- `ch6_file0`
- `ch6_file1`
- `ch6_file2`
- `ch6_file3`
- `ch6_usertest`

These tests cover file creation, file reading and writing, file status query,
hard links, unlinking, and repeated open/unlink operations.

The most important checks are:

- `ch6_file1` checks `fstat`.
- `ch6_file2` checks `linkat`, `unlinkat`, and `nlink`.
- `ch6_file3` checks repeated file creation and deletion.

After these tests pass, the file-system system calls work correctly for the
required Chapter 6 scenarios.
