use crate::{imp, io};
use imp::fd::AsFd;

/// `copy_file_range(fd_in, off_in, fd_out, off_out, len, 0)`—Copies data
/// from one file to another.
///
/// # References
///  - [Linux]
///
/// [Linux]: https://man7.org/linux/man-pages/man2/copy_file_range.2.html
#[inline]
pub fn copy_file_range<InFd: AsFd, OutFd: AsFd>(
    fd_in: InFd,
    off_in: Option<&mut u64>,
    fd_out: OutFd,
    off_out: Option<&mut u64>,
    len: u64,
) -> io::Result<u64> {
    imp::fs::syscalls::copy_file_range(fd_in.as_fd(), off_in, fd_out.as_fd(), off_out, len)
}
