#[cfg(any(target_os = "android", target_os = "linux"))]
use crate::process::Pid;
use crate::{imp, io};

pub use imp::process::types::Resource;

/// `struct rlimit`—Current and maximum values used in [`getrlimit`],
/// [`setrlimit`], and [`prlimit`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rlimit {
    /// Current effective, "soft", limit.
    pub current: Option<u64>,
    /// Maximum, "hard", value that `current` may be dynamically increased to.
    pub maximum: Option<u64>,
}

/// `getrlimit(resource)`—Get a process resource limit value.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/getrlimit.html
/// [Linux]: https://man7.org/linux/man-pages/man2/getrlimit.2.html
#[inline]
pub fn getrlimit(resource: Resource) -> Rlimit {
    imp::process::syscalls::getrlimit(resource)
}

/// `setrlimit(resource, new)`—Set a process resource limit value.
///
/// # References
///  - [POSIX]
///  - [Linux]
///
/// [POSIX]: https://pubs.opengroup.org/onlinepubs/9699919799/functions/setrlimit.html
/// [Linux]: https://man7.org/linux/man-pages/man2/setrlimit.2.html
#[inline]
pub fn setrlimit(resource: Resource, new: Rlimit) -> io::Result<()> {
    imp::process::syscalls::setrlimit(resource, new)
}

/// `prlimit(pid, resource, new)`—Get and set a process resource limit value.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/prlimit.2.html
#[cfg(any(target_os = "android", target_os = "linux"))]
#[inline]
pub fn prlimit(pid: Option<Pid>, resource: Resource, new: Rlimit) -> io::Result<Rlimit> {
    imp::process::syscalls::prlimit(pid, resource, new)
}
