# utopia_sys

`utopia_sys` is a `no_std` Rust library for making system calls through the Utopia ABI. It provides low-level syscall wrappers and optional formatted output helpers.

To create a program first check how to do it by seeing how /utest works.

## Supported architectures

All supported by Utopia kernel (`riscv64`, `x86_64`)

## API

The `raw` bindings for the supported syscalls:

- `raw::_write(fd, buffer, length)` writes bytes to a file descriptor.
- `raw::_exit(code)` exits the current process and does not return.
- `raw::_spawn(path, argv, envp, convention)` starts a program.
- `raw::_become(path, argv, envp, convention)` replaces the current program.

The `wrapper` Wraps raw bindings for the supported syscalls in more seveloper friendly way:
- `wrapper::{writeln!, write!, print!, println!, eprint!, eprintln!}` writing functions.
- `wrapper:::exit(code)` exits the current process and does not return.
- `wrapper:::spawn(path, argv, envp, convention)` starts a program.
- `wrapper:::become(path, argv, envp, convention)` replaces the current program.


## Optional features

The default features are `wrapper` and `panic-handler`.

- `wrapper` enables the `wrapper` module and the `print!`, `println!`, `eprint!`, and `eprintln!` macros.
- `panic-handler` enables a panic handler.
