//! Minimal libc-like tree used to check how libc's item-group macros
//! (`s!`, `s2!`, ...) are expanded by `libc_cfg::search`.
//!
//! The deprecated definitions live in their own module so tests can check how
//! `deprecated_cfg` is split from `cfg`.

#[cfg(target_os = "fuchsia")]
mod fuchsia;

cfg_if! {
    if #[cfg(unix)] {
        mod unix;
        mod extra;
    }
}
