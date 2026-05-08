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
