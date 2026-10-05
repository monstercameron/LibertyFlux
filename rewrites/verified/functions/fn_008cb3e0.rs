// original: 0x008CB3E0 stream_channel_flag (proposed)

/// Reads the flag byte of streaming channel `index` from `TABLE`; indices
/// above 14 (including large unsigned values) yield 1 without reading.
///
/// One stack argument (cdecl); the byte result is returned in AL.
lf_checker_rt::export!(cdecl, rw_008CB3E0(index: u32) -> u32 {
    unsafe {
        /// Base of the per-channel flag bytes.
        const TABLE: u32 = 0x1172FD0;
        /// Highest index with its own flag byte.
        const MAX_INDEX: u32 = 14;
        /// Value returned for an out-of-range index.
        const CLAMPED: u32 = 1;
        if index > MAX_INDEX {
            CLAMPED
        } else {
            (lf_checker_rt::relocated(TABLE).wrapping_add(index) as *const u8).read() as u32
        }
    }
});
