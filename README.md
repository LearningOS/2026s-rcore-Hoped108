# rCore-Tutorial-Code

## Code

- [Soure Code of labs](https://github.com/LearningOS/rCore-Tutorial-Code)

## Documents

- Concise Manual: [rCore-Tutorial-Guide](https://LearningOS.github.io/rCore-Tutorial-Guide/)

- Detail Book [rCore-Tutorial-Book-v3](https://rcore-os.github.io/rCore-Tutorial-Book-v3/)

## OS API docs of rCore Tutorial Code

- [OS API docs of ch1](https://learningos.github.io/rCore-Tutorial-Code/ch1/os/index.html)
  AND [OS API docs of ch2](https://learningos.github.io/rCore-Tutorial-Code/ch2/os/index.html)
- [OS API docs of ch3](https://learningos.github.io/rCore-Tutorial-Code/ch3/os/index.html)
  AND [OS API docs of ch4](https://learningos.github.io/rCore-Tutorial-Code/ch4/os/index.html)
- [OS API docs of ch5](https://learningos.github.io/rCore-Tutorial-Code/ch5/os/index.html)
  AND [OS API docs of ch6](https://learningos.github.io/rCore-Tutorial-Code/ch6/os/index.html)
- [OS API docs of ch7](https://learningos.github.io/rCore-Tutorial-Code/ch7/os/index.html)
  AND [OS API docs of ch8](https://learningos.github.io/rCore-Tutorial-Code/ch8/os/index.html)
- [OS API docs of ch9](https://learningos.github.io/rCore-Tutorial-Code/ch9/os/index.html)

## Related Resources

- [Learning Resource](https://github.com/LearningOS/rust-based-os-comp2025/blob/main/relatedinfo.md)

## Setup

```bash
$ git clone https://github.com/LearningOS/2026s-rcore-[YOUR_USER_NAME].git
$ cd 2026s-rcore-[YOUR_USER_NAME]
```

## Build & Run

```bash
# setup build&run environment first
$ git clone https://github.com/LearningOS/rCore-Tutorial-Test.git user
$ git checkout ch$ID
$ cd os
# run OS in ch$ID
$ make run
```

If you want to use docker to build and run, you can use the following command:
```bash
# After clone the `rCore-Tutorial-Test` repository to your local machine, you can use the following command to build and run:
$ make build_docker
$ make docker
```

If you experience network issues when accessing foreign resources such as GitHub in Docker, you can follow the following suggestions according to your stage:

- Docker pull:
  1. use proxy: https://docs.docker.com/reference/cli/docker/image/pull/#proxy-configuration

  2. use available domestic source (self-search)

- Docker build: use proxy https://docs.docker.com/engine/cli/proxy/#build-with-a-proxy-configuration

- Docker run: use proxy option, related operations are similar to `Docker build`, can refer to the relevant materials by yourself


Notice: $ID is from [1-9]

## Kernel Inspection Tools

Reading the tutorial alone can make the boot process feel abstract. The tools
below help connect source files, linker symbols, ELF layout, and QEMU output.

For chapter 1, the main flow is:

```text
make run
  -> cargo builds os/
  -> linker.ld places the kernel at 0x80200000
  -> QEMU starts a virtual RISC-V machine
  -> OpenSBI runs first
  -> OpenSBI jumps to the kernel entry
  -> entry.asm::_start sets the stack
  -> main.rs::rust_main() runs
```

Build the kernel before inspecting it:

```bash
$ cd os
$ make build
```

The generated ELF kernel is usually:

```text
target/riscv64gc-unknown-none-elf/release/os
```

The generated raw binary is usually:

```text
target/riscv64gc-unknown-none-elf/release/os.bin
```

### Useful Commands

Check the ELF header and entry address:

```bash
$ rust-readobj -h target/riscv64gc-unknown-none-elf/release/os
```

Look for:

```text
Format: elf64-littleriscv
Machine: EM_RISCV
Entry: 0x80200000
```

Check section layout:

```bash
$ rust-readobj -S target/riscv64gc-unknown-none-elf/release/os
```

Useful sections to find:

```text
.text      code and instructions
.rodata    read-only data and string literals
.data      initialized global/static data
.bss       zero-initialized or uninitialized global/static data
.bss.stack boot stack reserved by entry.asm
```

Check linker and assembly symbols:

```bash
$ rust-readobj --symbols target/riscv64gc-unknown-none-elf/release/os | grep -E 'stext|etext|srodata|erodata|sdata|edata|sbss|ebss|boot_stack|rust_main|_start'
```

Useful symbols:

```text
_start                  first kernel instruction label from entry.asm
rust_main               Rust kernel entry function from main.rs
stext / etext           .text range from linker.ld
srodata / erodata       .rodata range from linker.ld
sdata / edata           .data range from linker.ld
sbss / ebss             .bss range from linker.ld
boot_stack_lower_bound  boot stack bottom from entry.asm
boot_stack_top          boot stack top from entry.asm
```

Disassemble the kernel:

```bash
$ rust-objdump --arch-name=riscv64 -d target/riscv64gc-unknown-none-elf/release/os | less
```

Near `_start`, you should see instructions corresponding to:

```asm
la sp, boot_stack_top
call rust_main
```

Use the project shortcut for disassembly:

```bash
$ make disasm
```

Search source code quickly:

```bash
$ rg "rust_main|clear_bss|sbss|boot_stack|console_putchar" os/src
```

Inspect dependencies:

```bash
$ cargo tree
```

This is useful for confirming that logging macros such as `trace!`, `debug!`,
`info!`, `warn!`, and `error!` come from the `log` crate.

Generate local Rust API documentation:

```bash
$ cargo doc --no-deps --open
```

The API docs are useful for browsing modules, functions, macros, and source
locations. They complement the tutorial book: the book explains the design,
while the API docs help inspect the actual Rust interfaces.

### Recommended Learning Order

```text
1. read linker.ld
2. rust-readobj -h       -> verify the ELF entry address
3. rust-readobj -S       -> verify .text/.rodata/.data/.bss layout
4. rust-readobj --symbols -> find _start, rust_main, sbss, ebss, boot stack
5. rust-objdump -d       -> inspect real RISC-V instructions
6. make run LOG=TRACE    -> compare runtime output with symbols and sections
```

## Chapter 3 Exercise Notes

The Chapter 3 exercise page asks you to implement syscall `410`, named
`sys_trace`, on branch `ch3`.

Exercise page:

- https://learningos.cn/rCore-Tutorial-Guide/chapter3/5exercise.html

The tests are user programs under:

```text
user/src/bin/
```

For Chapter 3, the most relevant files are:

```text
user/src/bin/ch3_sleep.rs
user/src/bin/ch3_sleep1.rs
user/src/bin/ch3_trace.rs
user/src/bin/ch3b_yield0.rs
user/src/bin/ch3b_yield1.rs
user/src/bin/ch3b_yield2.rs
```

The main exercise test is:

```text
user/src/bin/ch3_trace.rs
```

It checks `trace_read`, `trace_write`, and `count_syscall`.

Run only the Chapter 3 exercise tests:

```bash
$ git checkout ch3
$ cd os
$ make run BASE=0
```

Run both basic and exercise tests:

```bash
$ git checkout ch3
$ cd os
$ make run BASE=2
```

Plain `make run` uses the default `BASE=1` in `os/Makefile`, so it only runs
basic tests and may not test the exercise.

Meaning of `BASE`:

```text
BASE=0  exercise tests, such as ch3_trace.rs
BASE=1  basic tests, such as ch3b_yield0.rs
BASE=2  both exercise and basic tests
```

Kernel files usually involved in this exercise:

```text
os/src/syscall/mod.rs
os/src/syscall/process.rs
os/src/trap/mod.rs
os/src/task/mod.rs
os/src/task/task.rs
```

## Grading

```bash
# setup build&run environment first
$ rm -rf ci-user
$ git clone https://github.com/LearningOS/rCore-Tutorial-Checker.git ci-user
$ git clone https://github.com/LearningOS/rCore-Tutorial-Test.git ci-user/user
$ git checkout ch$ID
# check&grade OS in ch$ID with more tests
$ cd ci-user && make test CHAPTER=$ID
```

Notice: $ID is from [3,4,5,6,8]
