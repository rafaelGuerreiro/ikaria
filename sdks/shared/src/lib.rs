pub mod protocol {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct WorldTick {
        pub tick: u64,
    }
}
