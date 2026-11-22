use core::arch::asm;

#[cfg(target_arch = "riscv64")]
#[inline(always)]
pub fn syscall_arch(
    num: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> isize {
    let ret: isize;
    unsafe {
        asm!("ecall", inlateout("a0") num => ret, in("a1") a1, in("a2") a2, in("a3") a3, in("a4") a4, in("a5") a5, in("a7") a6, options(nostack, preserves_flags));
    }
    ret
}

#[cfg(target_arch = "x86_64")]
#[inline(always)]
pub fn syscall_arch(
    num: usize,
    a1: usize,
    a2: usize,
    a3: usize,
    a4: usize,
    a5: usize,
    a6: usize,
) -> isize {
    let ret: isize;
    unsafe {
        asm!(
            "syscall",
            inlateout("rax") num => ret,
            in("rdx") a1,
            in("rdi") a2,
            in("rsi") a3,
            in("r10") a4,
            in("r9") a5,
            in("r8") a6,
            out("rcx") _,
            out("r11") _,
            options(nostack, preserves_flags)
        );
    }
    ret
}
