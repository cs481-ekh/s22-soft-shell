# bsu_st_interpreter

[![build and test workflow status badge](https://github.com/GoWestRobotics/bsu_st_interpreter/actions/workflows/main.yml/badge.svg)](https://github.com/GoWestRobotics/bsu_st_interpreter/actions/workflows/main.yml)

BSU Capstone senior project - Structured Text Interpreter

## Dev Environment Setup

Install cargo-c into cargo for automatic C header file and build management:

```shell
$ cargo install cargo-c
```

It then can be built by calling:

```shell
$ cargo cbuild
```


Tests can be run by calling:

```shell
$ cargo ctest
```

Ref: https://crates.io/crates/cargo-c
