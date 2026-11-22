use crate::syscall::syscall_arch;
use crate::{ErrorCodes, SyscallConv, SyscallNumber};
use core::ffi::CStr;

#[inline]
fn decode_ret(ret: isize) -> Result<usize, ErrorCodes> {
    if ret < 0 {
        Err(ErrorCodes::from(ret))
    } else {
        Ok(ret as usize)
    }
}

const EXEC_MAX_ARGS: usize = 1024;
const EXEC_MAX_PATH: usize = 4096;
const EXEC_MAX_ARG_BYTES: usize = 256 * 1024;

fn exec_syscall(
    number: SyscallNumber,
    path: &CStr,
    argv: &[&CStr],
    envp: &[&CStr],
    conv: SyscallConv,
) -> Result<usize, ErrorCodes> {
    if path.to_bytes_with_nul().len() > EXEC_MAX_PATH
        || argv.len() > EXEC_MAX_ARGS
        || envp.len() > EXEC_MAX_ARGS
    {
        return Err(ErrorCodes::MessageTooLarge);
    }

    let mut remaining_arg_bytes = EXEC_MAX_ARG_BYTES;
    for arg in argv.iter().chain(envp) {
        let arg_bytes = arg.to_bytes_with_nul().len();
        if arg_bytes > remaining_arg_bytes {
            return Err(ErrorCodes::MessageTooLarge);
        }
        remaining_arg_bytes -= arg_bytes;
    }

    let mut argv_ptrs = [core::ptr::null(); EXEC_MAX_ARGS + 1];
    let mut envp_ptrs = [core::ptr::null(); EXEC_MAX_ARGS + 1];
    for (dst, src) in argv_ptrs.iter_mut().zip(argv) {
        *dst = src.as_ptr();
    }
    for (dst, src) in envp_ptrs.iter_mut().zip(envp) {
        *dst = src.as_ptr();
    }

    decode_ret(syscall_arch(
        number as usize,
        path.as_ptr() as usize,
        conv as usize,
        argv_ptrs.as_ptr() as usize,
        envp_ptrs.as_ptr() as usize,
        0,
        0,
    ))
}

/// Syscall - 5
/// write - writes arg3 (len) amount of bytes from arg2 (buf) to the file described by arg1 (fd)
pub fn _write(fd: usize, buf: &[u8]) -> Result<usize, ErrorCodes> {
    decode_ret(syscall_arch(
        SyscallNumber::Write as usize,
        fd,
        buf.as_ptr() as usize,
        buf.len(),
        0,
        0,
        0,
    ))
}

/// Syscall - 2000
/// exit - exits the current process with the exit code arg1 (code)
pub fn _exit(code: usize) -> ! {
    syscall_arch(SyscallNumber::Exit as usize, code, 0, 0, 0, 0, 0);
    loop {
        core::hint::spin_loop();
    }
}

/// Syscall - 2001
/// spawns a process with an executable from arg1 (path) using arg2 (conv) syscall convention (where 0 means default and 1 means Linux).
/// arg3 (argv) is a pointer to Unix-like NUL-terminated argv; arg4 is envp.
pub fn _spawn(
    path: &CStr,
    argv: &[&CStr],
    envp: &[&CStr],
    conv: SyscallConv,
) -> Result<usize, ErrorCodes> {
    exec_syscall(SyscallNumber::Spawn, path, argv, envp, conv)
}

/// Syscall - 2002
/// replaces the current process with an executable from path arg1 (path) using arg2 (conv) syscall convention.
/// arg3 (argv) s a pointer to Unix-like NUL-terminated argv; and arg4 is envp.
pub fn _become(
    path: &CStr,
    argv: &[&CStr],
    envp: &[&CStr],
    conv: SyscallConv,
) -> Result<usize, ErrorCodes> {
    exec_syscall(SyscallNumber::Become, path, argv, envp, conv)
}
