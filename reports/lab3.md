# Lab 3 Report: Process Management and Stride Scheduling

## Overview

This lab extends the rCore kernel with Chapter 5 process-management features.
The main work is the implementation of stride scheduling, together with the
supporting `set_priority` system call. I also completed the `spawn` interface
needed by the user tests.

Because the guide requires all reports up to this chapter, the `reports`
directory now contains `lab1.md`, `lab2.md`, and `lab3.md`.

The final test result was:

```text
Test passed: 15/15
```

## Stride Scheduling

The original task manager used a simple FIFO ready queue. A task was pushed to
the back of the queue when it became ready, and the scheduler always popped the
front task. This behaves like round-robin scheduling.

For stride scheduling, each task keeps two scheduling fields:

```rust
priority: usize
stride: usize
```

The priority represents the task weight. A larger priority should receive more
CPU time. The stride records the accumulated virtual runtime of the task. The
scheduler always chooses the ready task with the smallest stride.

The stride increment is computed by:

```rust
BIG_STRIDE / priority
```

In this implementation, `BIG_STRIDE` is defined as:

```rust
pub const BIG_STRIDE: usize = 1_000_000;
```

A high-priority task has a smaller increment, so its stride grows more slowly.
As a result, it is selected more often over the long term.

## Scheduler Changes

The ready queue is still stored as:

```rust
VecDeque<Arc<TaskControlBlock>>
```

However, `TaskManager::fetch` no longer simply removes the front task. Instead,
it scans the ready queue, finds the task with the minimum stride, and removes
that task from the queue.

The current task's stride is updated when it gives up the CPU and is going back
to the ready queue. This is done in `suspend_current_and_run_next`, before
calling `add_task`.

This placement is important. `add_task` is also used for newly created tasks,
such as the initial process and spawned child processes. A new task has not used
CPU time yet, so its stride should not be increased just because it is inserted
into the ready queue.

When a task exits, it becomes a zombie and is not inserted back into the ready
queue. Therefore, `exit_current_and_run_next` does not need to update its
stride.

## Priority System Call

The `set_priority` system call updates the current task's priority. Invalid
priorities are rejected. In this implementation, priorities less than or equal
to 1 return `-1`.

Changing priority does not reset the task's stride. This avoids giving a task an
unfair advantage by repeatedly changing its priority to erase its previous CPU
usage.

## Spawn

The `spawn` system call loads an application by name, creates a new
`TaskControlBlock`, connects it to the current task as a child, and inserts it
into the ready queue.

The new task starts with the default priority and an initial stride of zero.
After it actually runs and yields or is preempted, its stride is increased by
`BIG_STRIDE / priority`.

## Testing

The important user programs are:

- `ch5_setprio`: checks valid and invalid priority values.
- `ch5_stride`: starts six CPU-bound child programs.
- `ch5_stride0` to `ch5_stride5`: run with priorities from 5 to 10 and print
  their loop counts.

For the stride test, the expected behavior is that a larger priority produces a
larger loop count. The printed `ratio = count / priority` values should be
roughly close to each other, showing that CPU time is distributed in proportion
to priority.

The checker output reported:

```text
Test passed: 15/15
```

This confirms that the required Chapter 5 functionality passed the automated
tests.
