#[cfg(all(
    any(
        target_os = "android",
        target_os = "illumos",
        target_os = "linux",
        target_os = "redox",
    ),
    not(feature = "force-old-poll")
))]
mod epoll;

#[cfg(all(
    any(
        target_os = "android",
        target_os = "illumos",
        target_os = "linux",
        target_os = "redox",
    ),
    not(feature = "force-old-poll")
))]
pub(crate) use self::epoll::{event, Event, Events, Selector};

#[cfg(all(
    any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ),
    not(feature = "force-old-poll")
))]
mod kqueue;

#[cfg(all(
    any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    ),
    not(feature = "force-old-poll")
))]
pub(crate) use self::kqueue::{event, Event, Events, Selector};

#[cfg(any(
    not(any(
        target_os = "android",
        target_os = "illumos",
        target_os = "linux",
        target_os = "redox",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    )),
    feature = "force-old-poll"
))]
mod poll;

#[cfg(any(
    not(any(
        target_os = "android",
        target_os = "illumos",
        target_os = "linux",
        target_os = "redox",
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "netbsd",
        target_os = "openbsd"
    )),
    feature = "force-old-poll"
))]
pub(crate) use self::poll::{event, Event, Events, Selector};

/// Lowest file descriptor used in `Selector::try_clone`.
///
/// # Notes
///
/// Usually fds 0, 1 and 2 are standard in, out and error. Some application
/// blindly assume this to be true, which means using any one of those a select
/// could result in some interesting and unexpected errors. Avoid that by using
/// an fd that doesn't have a pre-determined usage.
#[cfg(not(feature = "force-old-poll"))]
const LOWEST_FD: libc::c_int = 3;
