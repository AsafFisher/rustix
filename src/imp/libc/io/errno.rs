//! The `rustix` `Errno` type.
//!
//! This type holds an OS error code, which conceptually corresponds to an
//! `errno` value.

use super::super::c;
use libc_errno::errno;

/// The error type for `rustix` APIs.
///
/// This is similar to `std::io::Error`, but only holds an OS error code,
/// and no extra error value.
#[repr(transparent)]
#[doc(alias = "errno")]
#[derive(Eq, PartialEq, Hash, Copy, Clone)]
pub struct Errno(pub(crate) c::c_int);

impl Errno {
    /// `EACCES`
    #[doc(alias = "ACCES")]
    pub const ACCESS: Self = Self(c::EACCES);
    /// `EADDRINUSE`
    pub const ADDRINUSE: Self = Self(c::EADDRINUSE);
    /// `EADDRNOTAVAIL`
    pub const ADDRNOTAVAIL: Self = Self(c::EADDRNOTAVAIL);
    /// `EADV`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const ADV: Self = Self(c::EADV);
    /// `EAFNOSUPPORT`
    pub const AFNOSUPPORT: Self = Self(c::EAFNOSUPPORT);
    /// `EAGAIN`
    pub const AGAIN: Self = Self(c::EAGAIN);
    /// `EALREADY`
    pub const ALREADY: Self = Self(c::EALREADY);
    /// `EAUTH`
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const AUTH: Self = Self(c::EAUTH);
    /// `EBADE`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const BADE: Self = Self(c::EBADE);
    /// `EBADF`
    pub const BADF: Self = Self(c::EBADF);
    /// `EBADFD`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const BADFD: Self = Self(c::EBADFD);
    /// `EBADMSG`
    #[cfg(not(windows))]
    pub const BADMSG: Self = Self(c::EBADMSG);
    /// `EBADR`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const BADR: Self = Self(c::EBADR);
    /// `EBADRPC`
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const BADRPC: Self = Self(c::EBADRPC);
    /// `EBADRQC`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const BADRQC: Self = Self(c::EBADRQC);
    /// `EBADSLT`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const BADSLT: Self = Self(c::EBADSLT);
    /// `EBFONT`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const BFONT: Self = Self(c::EBFONT);
    /// `EBUSY`
    #[cfg(not(windows))]
    pub const BUSY: Self = Self(c::EBUSY);
    /// `ECANCELED`
    pub const CANCELED: Self = Self(c::ECANCELED);
    /// `ECAPMODE`
    #[cfg(any(target_os = "freebsd"))]
    pub const CAPMODE: Self = Self(c::ECAPMODE);
    /// `ECHILD`
    #[cfg(not(windows))]
    pub const CHILD: Self = Self(c::ECHILD);
    /// `ECHRNG`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const CHRNG: Self = Self(c::ECHRNG);
    /// `ECOMM`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const COMM: Self = Self(c::ECOMM);
    /// `ECONNABORTED`
    pub const CONNABORTED: Self = Self(c::ECONNABORTED);
    /// `ECONNREFUSED`
    pub const CONNREFUSED: Self = Self(c::ECONNREFUSED);
    /// `ECONNRESET`
    pub const CONNRESET: Self = Self(c::ECONNRESET);
    /// `EDEADLK`
    #[cfg(not(windows))]
    pub const DEADLK: Self = Self(c::EDEADLK);
    /// `EDEADLOCK`
    #[cfg(not(any(
        windows,
        target_os = "android",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const DEADLOCK: Self = Self(c::EDEADLOCK);
    /// `EDESTADDRREQ`
    pub const DESTADDRREQ: Self = Self(c::EDESTADDRREQ);
    /// `EDISCON`
    #[cfg(windows)]
    pub const DISCON: Self = Self(c::EDISCON);
    /// `EDOM`
    #[cfg(not(windows))]
    pub const DOM: Self = Self(c::EDOM);
    /// `EDOOFUS`
    #[cfg(any(target_os = "dragonfly", target_os = "freebsd"))]
    pub const DOOFUS: Self = Self(c::EDOOFUS);
    /// `EDOTDOT`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "illumos",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const DOTDOT: Self = Self(c::EDOTDOT);
    /// `EDQUOT`
    pub const DQUOT: Self = Self(c::EDQUOT);
    /// `EEXIST`
    #[cfg(not(windows))]
    pub const EXIST: Self = Self(c::EEXIST);
    /// `EFAULT`
    pub const FAULT: Self = Self(c::EFAULT);
    /// `EFBIG`
    #[cfg(not(windows))]
    pub const FBIG: Self = Self(c::EFBIG);
    /// `EFTYPE`
    #[cfg(any(
        target_env = "newlib",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const FTYPE: Self = Self(c::EFTYPE);
    /// `EHOSTDOWN`
    #[cfg(not(target_os = "wasi"))]
    pub const HOSTDOWN: Self = Self(c::EHOSTDOWN);
    /// `EHOSTUNREACH`
    pub const HOSTUNREACH: Self = Self(c::EHOSTUNREACH);
    /// `EHWPOISON`
    #[cfg(not(any(
        windows,
        target_os = "android",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "illumos",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "redox",
        target_os = "wasi",
    )))]
    pub const HWPOISON: Self = Self(c::EHWPOISON);
    /// `EIDRM`
    #[cfg(not(windows))]
    pub const IDRM: Self = Self(c::EIDRM);
    /// `EILSEQ`
    #[cfg(not(windows))]
    pub const ILSEQ: Self = Self(c::EILSEQ);
    /// `EINPROGRESS`
    pub const INPROGRESS: Self = Self(c::EINPROGRESS);
    /// `EINTR`
    ///
    /// For a convenient way to retry system calls that exit with `INTR`, use
    /// [`with_retrying`].
    ///
    /// [`with_retrying`]: crate::io::with_retrying
    pub const INTR: Self = Self(c::EINTR);
    /// `EINVAL`
    pub const INVAL: Self = Self(c::EINVAL);
    /// `EINVALIDPROCTABLE`
    #[cfg(windows)]
    pub const INVALIDPROCTABLE: Self = Self(c::EINVALIDPROCTABLE);
    /// `EINVALIDPROVIDER`
    #[cfg(windows)]
    pub const INVALIDPROVIDER: Self = Self(c::EINVALIDPROVIDER);
    /// `EIO`
    #[cfg(not(windows))]
    pub const IO: Self = Self(c::EIO);
    /// `EISCONN`
    pub const ISCONN: Self = Self(c::EISCONN);
    /// `EISDIR`
    #[cfg(not(windows))]
    pub const ISDIR: Self = Self(c::EISDIR);
    /// `EISNAM`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "illumos",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const ISNAM: Self = Self(c::EISNAM);
    /// `EKEYEXPIRED`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "illumos",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const KEYEXPIRED: Self = Self(c::EKEYEXPIRED);
    /// `EKEYREJECTED`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "illumos",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const KEYREJECTED: Self = Self(c::EKEYREJECTED);
    /// `EKEYREVOKED`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "illumos",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const KEYREVOKED: Self = Self(c::EKEYREVOKED);
    /// `EL2HLT`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const L2HLT: Self = Self(c::EL2HLT);
    /// `EL2NSYNC`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const L2NSYNC: Self = Self(c::EL2NSYNC);
    /// `EL3HLT`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const L3HLT: Self = Self(c::EL3HLT);
    /// `EL3RST`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const L3RST: Self = Self(c::EL3RST);
    /// `ELIBACC`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const LIBACC: Self = Self(c::ELIBACC);
    /// `ELIBBAD`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const LIBBAD: Self = Self(c::ELIBBAD);
    /// `ELIBEXEC`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const LIBEXEC: Self = Self(c::ELIBEXEC);
    /// `ELIBMAX`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const LIBMAX: Self = Self(c::ELIBMAX);
    /// `ELIBSCN`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const LIBSCN: Self = Self(c::ELIBSCN);
    /// `ELNRNG`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const LNRNG: Self = Self(c::ELNRNG);
    /// `ELOOP`
    pub const LOOP: Self = Self(c::ELOOP);
    /// `EMEDIUMTYPE`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "illumos",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const MEDIUMTYPE: Self = Self(c::EMEDIUMTYPE);
    /// `EMFILE`
    pub const MFILE: Self = Self(c::EMFILE);
    /// `EMLINK`
    #[cfg(not(windows))]
    pub const MLINK: Self = Self(c::EMLINK);
    /// `EMSGSIZE`
    pub const MSGSIZE: Self = Self(c::EMSGSIZE);
    /// `EMULTIHOP`
    #[cfg(not(any(windows, target_os = "openbsd")))]
    pub const MULTIHOP: Self = Self(c::EMULTIHOP);
    /// `ENAMETOOLONG`
    pub const NAMETOOLONG: Self = Self(c::ENAMETOOLONG);
    /// `ENAVAIL`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "illumos",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NAVAIL: Self = Self(c::ENAVAIL);
    /// `ENEEDAUTH`
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const NEEDAUTH: Self = Self(c::ENEEDAUTH);
    /// `ENETDOWN`
    pub const NETDOWN: Self = Self(c::ENETDOWN);
    /// `ENETRESET`
    pub const NETRESET: Self = Self(c::ENETRESET);
    /// `ENETUNREACH`
    pub const NETUNREACH: Self = Self(c::ENETUNREACH);
    /// `ENFILE`
    #[cfg(not(windows))]
    pub const NFILE: Self = Self(c::ENFILE);
    /// `ENOANO`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOANO: Self = Self(c::ENOANO);
    /// `ENOATTR`
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const NOATTR: Self = Self(c::ENOATTR);
    /// `ENOBUFS`
    pub const NOBUFS: Self = Self(c::ENOBUFS);
    /// `ENOCSI`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOCSI: Self = Self(c::ENOCSI);
    /// `ENODATA`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NODATA: Self = Self(c::ENODATA);
    /// `ENODEV`
    #[cfg(not(windows))]
    pub const NODEV: Self = Self(c::ENODEV);
    /// `ENOENT`
    #[cfg(not(windows))]
    pub const NOENT: Self = Self(c::ENOENT);
    /// `ENOEXEC`
    #[cfg(not(windows))]
    pub const NOEXEC: Self = Self(c::ENOEXEC);
    /// `ENOKEY`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "illumos",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOKEY: Self = Self(c::ENOKEY);
    /// `ENOLCK`
    #[cfg(not(windows))]
    pub const NOLCK: Self = Self(c::ENOLCK);
    /// `ENOLINK`
    #[cfg(not(any(windows, target_os = "openbsd")))]
    pub const NOLINK: Self = Self(c::ENOLINK);
    /// `ENOMEDIUM`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "illumos",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOMEDIUM: Self = Self(c::ENOMEDIUM);
    /// `ENOMEM`
    #[cfg(not(windows))]
    pub const NOMEM: Self = Self(c::ENOMEM);
    /// `ENOMORE`
    #[cfg(windows)]
    pub const NOMORE: Self = Self(c::ENOMORE);
    /// `ENOMSG`
    #[cfg(not(windows))]
    pub const NOMSG: Self = Self(c::ENOMSG);
    /// `ENONET`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NONET: Self = Self(c::ENONET);
    /// `ENOPKG`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOPKG: Self = Self(c::ENOPKG);
    /// `ENOPROTOOPT`
    pub const NOPROTOOPT: Self = Self(c::ENOPROTOOPT);
    /// `ENOSPC`
    #[cfg(not(windows))]
    pub const NOSPC: Self = Self(c::ENOSPC);
    /// `ENOSR`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOSR: Self = Self(c::ENOSR);
    /// `ENOSTR`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOSTR: Self = Self(c::ENOSTR);
    /// `ENOSYS`
    #[cfg(not(windows))]
    pub const NOSYS: Self = Self(c::ENOSYS);
    /// `ENOTBLK`
    #[cfg(not(any(windows, target_os = "wasi")))]
    pub const NOTBLK: Self = Self(c::ENOTBLK);
    /// `ENOTCAPABLE`
    #[cfg(any(target_os = "freebsd", target_os = "wasi"))]
    pub const NOTCAPABLE: Self = Self(c::ENOTCAPABLE);
    /// `ENOTCONN`
    pub const NOTCONN: Self = Self(c::ENOTCONN);
    /// `ENOTDIR`
    #[cfg(not(windows))]
    pub const NOTDIR: Self = Self(c::ENOTDIR);
    /// `ENOTEMPTY`
    pub const NOTEMPTY: Self = Self(c::ENOTEMPTY);
    /// `ENOTNAM`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "illumos",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOTNAM: Self = Self(c::ENOTNAM);
    /// `ENOTRECOVERABLE`
    #[cfg(not(any(windows, target_os = "dragonfly", target_os = "netbsd")))]
    pub const NOTRECOVERABLE: Self = Self(c::ENOTRECOVERABLE);
    /// `ENOTSOCK`
    pub const NOTSOCK: Self = Self(c::ENOTSOCK);
    /// `ENOTSUP`
    #[cfg(not(any(windows, target_os = "redox")))]
    pub const NOTSUP: Self = Self(c::ENOTSUP);
    /// `ENOTTY`
    #[cfg(not(windows))]
    pub const NOTTY: Self = Self(c::ENOTTY);
    /// `ENOTUNIQ`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const NOTUNIQ: Self = Self(c::ENOTUNIQ);
    /// `ENXIO`
    #[cfg(not(windows))]
    pub const NXIO: Self = Self(c::ENXIO);
    /// `EOPNOTSUPP`
    pub const OPNOTSUPP: Self = Self(c::EOPNOTSUPP);
    /// `EOVERFLOW`
    #[cfg(not(windows))]
    pub const OVERFLOW: Self = Self(c::EOVERFLOW);
    /// `EOWNERDEAD`
    #[cfg(not(any(windows, target_os = "dragonfly", target_os = "netbsd")))]
    pub const OWNERDEAD: Self = Self(c::EOWNERDEAD);
    /// `EPERM`
    #[cfg(not(windows))]
    pub const PERM: Self = Self(c::EPERM);
    /// `EPFNOSUPPORT`
    #[cfg(not(target_os = "wasi"))]
    pub const PFNOSUPPORT: Self = Self(c::EPFNOSUPPORT);
    /// `EPIPE`
    #[cfg(not(windows))]
    pub const PIPE: Self = Self(c::EPIPE);
    /// `EPROCLIM`
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const PROCLIM: Self = Self(c::EPROCLIM);
    /// `EPROCUNAVAIL`
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const PROCUNAVAIL: Self = Self(c::EPROCUNAVAIL);
    /// `EPROGMISMATCH`
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const PROGMISMATCH: Self = Self(c::EPROGMISMATCH);
    /// `EPROGUNAVAIL`
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const PROGUNAVAIL: Self = Self(c::EPROGUNAVAIL);
    /// `EPROTO`
    #[cfg(not(windows))]
    pub const PROTO: Self = Self(c::EPROTO);
    /// `EPROTONOSUPPORT`
    pub const PROTONOSUPPORT: Self = Self(c::EPROTONOSUPPORT);
    /// `EPROTOTYPE`
    pub const PROTOTYPE: Self = Self(c::EPROTOTYPE);
    /// `EPROVIDERFAILEDINIT`
    #[cfg(windows)]
    pub const PROVIDERFAILEDINIT: Self = Self(c::EPROVIDERFAILEDINIT);
    /// `ERANGE`
    #[cfg(not(windows))]
    pub const RANGE: Self = Self(c::ERANGE);
    /// `EREFUSED`
    #[cfg(windows)]
    pub const REFUSED: Self = Self(c::EREFUSED);
    /// `EREMCHG`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const REMCHG: Self = Self(c::EREMCHG);
    /// `EREMOTE`
    #[cfg(not(target_os = "wasi"))]
    pub const REMOTE: Self = Self(c::EREMOTE);
    /// `EREMOTEIO`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "illumos",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const REMOTEIO: Self = Self(c::EREMOTEIO);
    /// `ERESTART`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const RESTART: Self = Self(c::ERESTART);
    /// `ERFKILL`
    #[cfg(not(any(
        windows,
        target_os = "android",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "illumos",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "redox",
        target_os = "wasi",
    )))]
    pub const RFKILL: Self = Self(c::ERFKILL);
    /// `EROFS`
    #[cfg(not(windows))]
    pub const ROFS: Self = Self(c::EROFS);
    /// `ERPCMISMATCH`
    #[cfg(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ))]
    pub const RPCMISMATCH: Self = Self(c::ERPCMISMATCH);
    /// `ESHUTDOWN`
    #[cfg(not(target_os = "wasi"))]
    pub const SHUTDOWN: Self = Self(c::ESHUTDOWN);
    /// `ESOCKTNOSUPPORT`
    #[cfg(not(target_os = "wasi"))]
    pub const SOCKTNOSUPPORT: Self = Self(c::ESOCKTNOSUPPORT);
    /// `ESPIPE`
    #[cfg(not(windows))]
    pub const SPIPE: Self = Self(c::ESPIPE);
    /// `ESRCH`
    #[cfg(not(windows))]
    pub const SRCH: Self = Self(c::ESRCH);
    /// `ESRMNT`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const SRMNT: Self = Self(c::ESRMNT);
    /// `ESTALE`
    pub const STALE: Self = Self(c::ESTALE);
    /// `ESTRPIPE`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const STRPIPE: Self = Self(c::ESTRPIPE);
    /// `ETIME`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const TIME: Self = Self(c::ETIME);
    /// `ETIMEDOUT`
    pub const TIMEDOUT: Self = Self(c::ETIMEDOUT);
    /// `E2BIG`
    #[cfg(not(windows))]
    #[doc(alias = "2BIG")]
    pub const TOOBIG: Self = Self(c::E2BIG);
    /// `ETOOMANYREFS`
    #[cfg(not(target_os = "wasi"))]
    pub const TOOMANYREFS: Self = Self(c::ETOOMANYREFS);
    /// `ETXTBSY`
    #[cfg(not(windows))]
    pub const TXTBSY: Self = Self(c::ETXTBSY);
    /// `EUCLEAN`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "illumos",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const UCLEAN: Self = Self(c::EUCLEAN);
    /// `EUNATCH`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const UNATCH: Self = Self(c::EUNATCH);
    /// `EUSERS`
    #[cfg(not(target_os = "wasi"))]
    pub const USERS: Self = Self(c::EUSERS);
    /// `EWOULDBLOCK`
    pub const WOULDBLOCK: Self = Self(c::EWOULDBLOCK);
    /// `EXDEV`
    #[cfg(not(windows))]
    pub const XDEV: Self = Self(c::EXDEV);
    /// `EXFULL`
    #[cfg(not(any(
        windows,
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd",
        target_os = "wasi",
    )))]
    pub const XFULL: Self = Self(c::EXFULL);
}

impl Errno {
    /// Extract an `Errno` value from a `std::io::Error`.
    ///
    /// This isn't a `From` conversion because it's expected to be relatively
    /// uncommon.
    #[cfg(feature = "std")]
    #[inline]
    pub fn from_io_error(io_err: &std::io::Error) -> Option<Self> {
        io_err
            .raw_os_error()
            .and_then(|raw| if raw != 0 { Some(Self(raw)) } else { None })
    }

    /// Extract the raw OS error number from this error.
    #[inline]
    pub const fn raw_os_error(self) -> i32 {
        self.0
    }

    /// Construct an `Errno` from a raw OS error number.
    #[inline]
    pub const fn from_raw_os_error(raw: i32) -> Self {
        Self(raw)
    }

    pub(crate) fn last_os_error() -> Self {
        Self(errno().0)
    }
}
