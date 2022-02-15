# bsu_st_interpreter
BSU Capstone senior project - Structured Text Interpreter

## Dev Environment Setup

Install cbindgen into cargo for automatic C header file generation:

```shell
$ cargo install --force cbindgen
```

It then can be run by calling:

```shell
$ cbindgen --config cbindgen.toml --lang c --crate st_interpret --output st_interpret.h
```

Src: https://github.com/eqrion/cbindgen