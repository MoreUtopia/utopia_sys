#![no_std]

#[cfg(not(all(
    target_os = "none",
    any(target_arch = "x86_64", target_arch = "riscv64")
)))]
compile_error!(
    "This library only supports x86_64-unknown-none and riscv64gc-unknown-none-elf which are only supported by utopia kernel targets."
);

#[cfg(feature = "panic-handler")]
/// Very simple panic handler for programs to work
pub mod panic;

/// Raw bindings for the supported syscalls
pub mod raw;

#[cfg(feature = "wrapper")]
/// Wrappers around raw bindings for the supported syscalls
pub mod wrapper;

#[cfg(feature = "start")]
/// Implements _start for program to work
pub mod start;

mod syscall;

pub mod prelude {
    pub use crate::ErrorCodes;
    pub use crate::SyscallConv;
    pub use crate::wrapper::*;
}

/// Syscall convention for Utopia
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallConv {
    Own = 0,
    Linux = 1,
}

/// Syscall number - same for x86_64 and RISC-V64
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SyscallNumber {
    Write = 5,
    Exit = 2000,
    Spawn = 2001,
    Become = 2002,
}

/// Error codes from Utopia Kernel
/// <https://github.com/gorciu-official/Utopia/blob/main/docs/syscalls.md>
#[repr(isize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCodes {
    /// ERR_NOT_IMPLEMENTED -1
    NotImplemented = -1,
    /// ERR_INVALID_ARGUMENT -2
    InvalidArgument = -2,
    /// ERR_NO_FILE -3
    NoFile = -3,
    /// ERR_BAD_FS_NODE_TYPE -4
    BadFsNodeType = -4,
    /// ERR_PERMISSION_DENIED -5
    PermissionDenied = -5,
    /// ERR_BUFFER_TOO_SMALL -6
    BufferTooSmall = -6,
    /// ERR_MESSAGE_TOO_LARGE -7
    MessageTooLarge = -7,
    /// ERR_BAD_EXEC -8
    BadExec = -8,
    /// ERR_NO_MEMORY -9
    NoMemory = -9,
}

impl From<isize> for ErrorCodes {
    fn from(value: isize) -> Self {
        match value {
            -1 => Self::NotImplemented,
            -2 => Self::InvalidArgument,
            -3 => Self::NoFile,
            -4 => Self::BadFsNodeType,
            -5 => Self::PermissionDenied,
            -6 => Self::BufferTooSmall,
            -7 => Self::MessageTooLarge,
            -8 => Self::BadExec,
            -9 => Self::NoMemory,
            _ => panic!("unknown error code: {value}"),
        }
    }
}
