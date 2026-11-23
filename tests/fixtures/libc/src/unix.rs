s2! {
    #[exhaustive]
    pub struct rlimit {
        pub rlim_cur: u64,
        pub rlim_max: u64,
    }
}

pub const MIXED: u8 = 1;
