//! Architecture-specific syscall code.
//!
//! `rustix` has inline assembly sequences using `asm!`, but that requires
//! nightly Rust, so it also has out-of-line ("outline") assembly sequences
//! in .s files. And 32-bit x86 is special (see comments below).
//!
//! This module also has a `choose` submodule which chooses a scheme and is
//! what most of the `rustix` syscalls use.
//!
//! # Safety
//!
//! This contains the inline `asm` statements performing the syscall
//! instructions and FFI declarations declaring the out-of-line ("outline")
//! syscall instructions.

#![allow(unsafe_code)]
#![cfg_attr(not(feature = "all-apis"), allow(unused_imports))]

// When inline asm is available, use it. Otherwise, use out-of-line asm. These
// functions always use the machine's syscall instruction, even when it isn't
// the fastest option available.
#[cfg_attr(asm, path = "inline/mod.rs")]
#[cfg_attr(not(asm), path = "outline/mod.rs")]
pub(in crate::imp) mod asm;

// On most architectures, the architecture syscall instruction is fast, so use
// it directly.
#[cfg(any(
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "powerpc64",
    target_arch = "riscv64",
    target_arch = "x86_64",
))]
pub(in crate::imp) use self::asm as choose;

// On 32-bit x86, use vDSO wrappers for all syscalls. We could use the
// architecture syscall instruction (`int 0x80`), but the vDSO kernel_vsyscall
// mechanism is much faster.
#[cfg(target_arch = "x86")]
pub(in crate::imp) use super::vdso_wrappers::x86_via_vdso as choose;

// This would be the code for always using `int 0x80` on 32-bit x86.
//#[cfg(target_arch = "x86")]
//pub(in crate::imp) use self::asm as choose;

// Macros for invoking system calls.
//
// These factor out:
//  - Calling `nr` on the syscall number to convert it into `SyscallNumber`.
//  - Calling `.into()` on each of the arguments to convert them into `ArgReg`.
//  - Qualifying the `syscall*` and `__NR_*` identifiers.
//  - Counting the number of arguments.
macro_rules! syscall {
    ($nr:ident) => {
        $crate::imp::arch::choose::syscall0($crate::imp::reg::nr(linux_raw_sys::general::$nr))
    };

    ($nr:ident, $a0:expr) => {
        $crate::imp::arch::choose::syscall1(
            $crate::imp::reg::nr(linux_raw_sys::general::$nr),
            $a0.into(),
        )
    };

    ($nr:ident, $a0:expr, $a1:expr) => {
        $crate::imp::arch::choose::syscall2(
            $crate::imp::reg::nr(linux_raw_sys::general::$nr),
            $a0.into(),
            $a1.into(),
        )
    };

    ($nr:ident, $a0:expr, $a1:expr, $a2:expr) => {
        $crate::imp::arch::choose::syscall3(
            $crate::imp::reg::nr(linux_raw_sys::general::$nr),
            $a0.into(),
            $a1.into(),
            $a2.into(),
        )
    };

    ($nr:ident, $a0:expr, $a1:expr, $a2:expr, $a3:expr) => {
        $crate::imp::arch::choose::syscall4(
            $crate::imp::reg::nr(linux_raw_sys::general::$nr),
            $a0.into(),
            $a1.into(),
            $a2.into(),
            $a3.into(),
        )
    };

    ($nr:ident, $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        $crate::imp::arch::choose::syscall5(
            $crate::imp::reg::nr(linux_raw_sys::general::$nr),
            $a0.into(),
            $a1.into(),
            $a2.into(),
            $a3.into(),
            $a4.into(),
        )
    };

    ($nr:ident, $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        $crate::imp::arch::choose::syscall6(
            $crate::imp::reg::nr(linux_raw_sys::general::$nr),
            $a0.into(),
            $a1.into(),
            $a2.into(),
            $a3.into(),
            $a4.into(),
            $a5.into(),
        )
    };

    ($nr:ident, $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        $crate::imp::arch::choose::syscall7(
            $crate::imp::reg::nr(linux_raw_sys::general::$nr),
            $a0.into(),
            $a1.into(),
            $a2.into(),
            $a3.into(),
            $a4.into(),
            $a5.into(),
            $a6.into(),
        )
    };
}

macro_rules! syscall_readonly {
    ($nr:ident) => {
        $crate::imp::arch::choose::syscall0_readonly($crate::imp::reg::nr(
            linux_raw_sys::general::$nr,
        ))
    };

    ($nr:ident, $a0:expr) => {
        $crate::imp::arch::choose::syscall1_readonly(
            $crate::imp::reg::nr(linux_raw_sys::general::$nr),
            $a0.into(),
        )
    };

    ($nr:ident, $a0:expr, $a1:expr) => {
        $crate::imp::arch::choose::syscall2_readonly(
            $crate::imp::reg::nr(linux_raw_sys::general::$nr),
            $a0.into(),
            $a1.into(),
        )
    };

    ($nr:ident, $a0:expr, $a1:expr, $a2:expr) => {
        $crate::imp::arch::choose::syscall3_readonly(
            $crate::imp::reg::nr(linux_raw_sys::general::$nr),
            $a0.into(),
            $a1.into(),
            $a2.into(),
        )
    };

    ($nr:ident, $a0:expr, $a1:expr, $a2:expr, $a3:expr) => {
        $crate::imp::arch::choose::syscall4_readonly(
            $crate::imp::reg::nr(linux_raw_sys::general::$nr),
            $a0.into(),
            $a1.into(),
            $a2.into(),
            $a3.into(),
        )
    };

    ($nr:ident, $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr) => {
        $crate::imp::arch::choose::syscall5_readonly(
            $crate::imp::reg::nr(linux_raw_sys::general::$nr),
            $a0.into(),
            $a1.into(),
            $a2.into(),
            $a3.into(),
            $a4.into(),
        )
    };

    ($nr:ident, $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr) => {
        $crate::imp::arch::choose::syscall6_readonly(
            $crate::imp::reg::nr(linux_raw_sys::general::$nr),
            $a0.into(),
            $a1.into(),
            $a2.into(),
            $a3.into(),
            $a4.into(),
            $a5.into(),
        )
    };

    ($nr:ident, $a0:expr, $a1:expr, $a2:expr, $a3:expr, $a4:expr, $a5:expr, $a6:expr) => {
        $crate::imp::arch::choose::syscall7_readonly(
            $crate::imp::reg::nr(linux_raw_sys::general::$nr),
            $a0.into(),
            $a1.into(),
            $a2.into(),
            $a3.into(),
            $a4.into(),
            $a5.into(),
            $a6.into(),
        )
    };
}

#[cfg(feature = "runtime")]
macro_rules! syscall_noreturn {
    ($nr:ident, $a0:expr) => {
        $crate::imp::arch::choose::syscall1_noreturn(
            $crate::imp::reg::nr(linux_raw_sys::general::$nr),
            $a0.into(),
        )
    };
}
