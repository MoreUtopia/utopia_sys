use crate::{ErrorCodes, SyscallConv};
use core::ffi::CStr;
use core::fmt::{self, Write};

/// Exit current process with code.
pub fn exit(code: usize) -> ! {
    crate::raw::_exit(code)
}

/// Start a process
pub fn spawn(
    path: &CStr,
    argv: &[&CStr],
    envp: &[&CStr],
    conv: SyscallConv,
) -> Result<usize, ErrorCodes> {
    crate::raw::_spawn(path, argv, envp, conv)
}

/// Replace current process
pub fn becomes(
    path: &CStr,
    argv: &[&CStr],
    envp: &[&CStr],
    conv: SyscallConv,
) -> Result<usize, ErrorCodes> {
    crate::raw::_become(path, argv, envp, conv)
}

fn write_fd(fd: usize, bytes: &[u8]) -> fmt::Result {
    crate::raw::_write(fd, bytes)
        .map(|_| ())
        .map_err(|_| fmt::Error)
}

pub struct Stderr;

impl Write for Stderr {
    fn write_str(&mut self, message: &str) -> fmt::Result {
        write_fd(2, message.as_bytes())
    }
}

pub struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, message: &str) -> fmt::Result {
        write_fd(1, message.as_bytes())
    }
}

#[doc(hidden)]
pub fn print_to_stdout(args: fmt::Arguments<'_>) {
    let _ = Stdout.write_fmt(args);
}

#[doc(hidden)]
pub fn print_to_stderr(args: fmt::Arguments<'_>) {
    let _ = Stderr.write_fmt(args);
}

/// Print to stdout
#[macro_export]
macro_rules! print {
	($($arg:tt)*) => {
		$crate::wrapper::print_to_stdout(core::format_args!($($arg)*))
	};
}

/// Print to stdout with newline
#[macro_export]
macro_rules! println {
	() => { $crate::print!("\n") };
	($($arg:tt)*) => {
		$crate::print!("{}\n", core::format_args!($($arg)*))
	};
}

/// Print to stderr
#[macro_export]
macro_rules! eprint {
	($($arg:tt)*) => {
		$crate::wrapper::print_to_stderr(core::format_args!($($arg)*))
	};
}

/// Print to stderr with newline
#[macro_export]
macro_rules! eprintln {
	() => { $crate::eprint!("\n") };
	($($arg:tt)*) => {
		$crate::eprint!("{}\n", core::format_args!($($arg)*))
	};
}
