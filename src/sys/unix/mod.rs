/// Helper macro to execute a system call that returns an `io::Result`.
//
// Macro must be defined before any modules that uses them.
#[allow(unused_macros)]
macro_rules! syscall {
    ($fn: ident ( $($arg: expr),* $(,)* ) ) => {{
        let res = unsafe { libc::$fn($($arg, )*) };
        if res == -1 {
            Err(std::io::Error::last_os_error())
        } else {
            Ok(res)
        }
    }};
}

cfg_os_poll! {
    mod selector;
    pub(crate) use self::selector::{event, Event, Events, Selector, IoSourceState};

    mod sourcefd;
    pub use self::sourcefd::SourceFd;

    mod waker;
    pub(crate) use self::waker::Waker;

    cfg_net! {
        mod net;

        pub(crate) mod tcp;
        pub(crate) mod udp;
        pub(crate) mod uds;
        pub use self::uds::SocketAddr;
    }

    cfg_os_ext! {
        pub(crate) mod pipe;
    }
}

cfg_not_os_poll! {
    cfg_net! {
        mod uds;
        pub use self::uds::SocketAddr;
    }

    cfg_any_os_ext! {
        mod sourcefd;
        pub use self::sourcefd::SourceFd;
    }
}
