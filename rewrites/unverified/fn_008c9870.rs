// original: 0x008C9870 stream_channel_row (proposed)

/// Address of one streaming channel row: `TABLE + index * ROW_STRIDE` for an
/// index of 14 or less, otherwise `TABLE` itself (indices above 14, including
/// large unsigned values, clamp to the first row).
///
/// The computation reads no memory; it only derives an address from the
/// table base. One stack argument (cdecl), the address returned in EAX.
lf_checker_rt::export!(cdecl, rw_008C9870(index: u32) -> u32 {
    unsafe {
        /// Base of the channel row table.
        const TABLE: u32 = 0x1172D90;
        /// Highest index with its own row.
        const MAX_INDEX: u32 = 14;
        /// Row stride in bytes (index * 16 - index, doubled).
        const ROW_STRIDE: u32 = 30;
        let base = lf_checker_rt::relocated(TABLE);
        if index > MAX_INDEX {
            base
        } else {
            base.wrapping_add(index.wrapping_mul(ROW_STRIDE))
        }
    }
});
