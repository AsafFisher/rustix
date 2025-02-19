use crate::process::Pid;
use crate::{imp, io};

pub use imp::process::types::Signal;

/// `kill(pid, sig)`—Sends a signal to a process.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/kill.html
/// [Linux]: https://man7.org/linux/man-pages/man2/kill.2.html
#[inline]
#[doc(alias = "kill")]
pub fn kill_process(pid: Pid, sig: Signal) -> io::Result<()> {
    imp::process::syscalls::kill_process(pid, sig)
}

/// `kill(-pid, sig)`—Sends a signal to all processes in a process group.
///
/// If `pid` is 1, this sends a signal to all processes the current process
/// has permission to send signals to, except process `1`, possibly other
/// system-specific processes, and on some systems, the current process.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/kill.html
/// [Linux]: https://man7.org/linux/man-pages/man2/kill.2.html
#[inline]
#[doc(alias = "kill")]
pub fn kill_process_group(pid: Pid, sig: Signal) -> io::Result<()> {
    imp::process::syscalls::kill_process_group(pid, sig)
}

/// `kill(0, sig)`—Sends a signal to all processes in the current process
/// group.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/kill.html
/// [Linux]: https://man7.org/linux/man-pages/man2/kill.2.html
#[inline]
#[doc(alias = "kill")]
pub fn kill_current_process_group(sig: Signal) -> io::Result<()> {
    imp::process::syscalls::kill_current_process_group(sig)
}
