// original: 0x008CC660 stream_select_channel (proposed)

/// Selects streaming channel `index`: for an index of 14 or less loads the
/// channel word from `TABLE` into the global `CURRENT` word and returns it;
/// larger indices (including large unsigned values) leave the global alone
/// and return the index itself.
///
/// One stack argument (cdecl); the result is returned in EAX.
lf_checker_rt::export!(cdecl, rw_008CC660(index: u32) -> u32 {
    unsafe {
        /// Base of the per-channel words.
        const TABLE: u32 = 0x1172D50;
        /// Global currently selected channel word.
        const CURRENT: u32 = 0x1173240;
        /// Highest selectable index.
        const MAX_INDEX: u32 = 14;
        if index > MAX_INDEX {
            index
        } else {
            let v = (lf_checker_rt::relocated(TABLE).wrapping_add(index.wrapping_mul(4))
                as *const u32)
                .read();
            (lf_checker_rt::relocated(CURRENT) as *mut u32).write(v);
            v
        }
    }
});
