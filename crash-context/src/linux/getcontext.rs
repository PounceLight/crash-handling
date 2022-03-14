//! Inline implementation of [] since it is not supported on all targets, namely
//! musl, as it has been deprecated from POSIX for over a decade
//!
//! The implementation is ported from Breakpad

extern "C" {
    #[cfg_attr(target_arch = "aarch64", allow(improper_ctypes))]
    pub fn crash_context_getcontext(ctx: *mut super::ucontext_t) -> i32;
}

cfg_if::cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        mod x86_64;
    } else if #[cfg(target_arch = "x86")] {
        mod x86;
    } else if #[cfg(target_arch = "aarch64")] {
        mod aarch64;
    }
}
