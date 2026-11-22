//! Minimal libc-like tree used to check how libc's item-group macros
//! (`s!`, `s2!`, ...) are expanded by `libc_cfg::search`.

cfg_if! {
    if #[cfg(unix)] {
        mod unix;
        mod extra;
    }
}
