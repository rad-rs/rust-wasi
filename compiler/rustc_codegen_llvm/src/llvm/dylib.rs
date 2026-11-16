//! A thin wrapper around `libloading` for the optional dynamically loaded LLVM extensions
//! (enzyme, offload, tpde).
//!
//! On platforms without `dlopen` (e.g. wasm hosts, where rustc is itself compiled to
//! WASI) `libloading` has no `Library` type, so a stub that always fails to load is used
//! instead. On unix and windows this is exactly `libloading`.

#[cfg(any(unix, windows))]
pub(crate) use libloading::{Error, Library, Symbol};

#[cfg(not(any(unix, windows)))]
pub(crate) use self::unsupported::{Error, Library, Symbol};

#[cfg(not(any(unix, windows)))]
mod unsupported {
    use std::ffi::{OsStr, c_void};
    use std::fmt;
    use std::marker::PhantomData;
    use std::ops::Deref;

    #[derive(Debug)]
    pub(crate) struct Error;

    impl fmt::Display for Error {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("dynamic library loading is not supported on this platform")
        }
    }

    impl std::error::Error for Error {}

    pub(crate) struct Library;

    impl Library {
        pub(crate) unsafe fn new<P: AsRef<OsStr>>(_filename: P) -> Result<Library, Error> {
            Err(Error)
        }

        pub(crate) unsafe fn get<'lib, T>(
            &'lib self,
            _symbol: &[u8],
        ) -> Result<Symbol<'lib, T>, Error> {
            Err(Error)
        }
    }

    pub(crate) struct Symbol<'lib, T> {
        _marker: PhantomData<(&'lib Library, T)>,
    }

    impl<T> Symbol<'_, T> {
        pub(crate) fn try_as_raw_ptr(&self) -> Option<*mut c_void> {
            None
        }
    }

    impl<T> Deref for Symbol<'_, T> {
        type Target = T;

        fn deref(&self) -> &T {
            // A `Symbol` can never be created: `Library::get` always fails.
            unreachable!()
        }
    }
}
