# Lab 5 Report: Process Management and Stride Scheduling

## Overview

This lab is based on the `ch5` branch. The main work is to complete process
management features, including `spawn`, `set_priority`, and stride scheduling.

## Implementation

For `spawn`, the kernel loads the target application by name, creates a new
`TaskControlBlock`, sets its parent to the current process, adds it to the
current process's children list, and inserts it into the ready queue.

For `set_priority`, the kernel updates the current task's priority and rejects
invalid values. In my implementation, priorities less than or equal to `1`
return `-1`.

For stride scheduling, each task stores:

```rust
priority: usize
stride: usize
```

The scheduler chooses the ready task with the smallest `stride`. When a running
task gives up the CPU and returns to the ready queue, its stride is updated by:

```rust
BIG_STRIDE / priority
```

This makes high-priority tasks grow more slowly in stride, so they are selected
more often over time.

## Testing

I used the Chapter 5 tests, especially:

- `ch5_setprio`
- `ch5_stride`
- `ch5_stride0` to `ch5_stride5`

The checker result was:

```text
Test passed: 15/15
```

This shows that the Chapter 5 process and scheduling tests passed.
