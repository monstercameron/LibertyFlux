// original: 0x008CEB20 stream_bank_range_copy (proposed)

/// Copies a bank's range pair into `dst`: the bank pointer comes from
/// `TABLE[index]` and must be non-null, and the index `INVALID` (-90) is
/// rejected. On success writes the dwords at `+LO` and `+HI` to `dst[0]`
/// and `dst[1]`; on failure writes two zero dwords instead.
///
/// Two stack arguments (cdecl); no return value.
lf_checker_rt::export!(cdecl, rw_008CEB20(dst: u32, index: u32) -> u32 {
    unsafe {
        /// Base of the bank pointer table (one dword per index).
        const TABLE: u32 = 0x1173560;
        /// Offset of the range low dword in a bank object.
        const LO: u32 = 0x35D0;
        /// Offset of the range high dword in a bank object.
        const HI: u32 = 0x35D4;
        /// Rejected index.
        const INVALID: u32 = 0xFFFF_FFA6;
        let (lo, hi) = if index == INVALID {
            (0, 0)
        } else {
            let bank = (lf_checker_rt::relocated(TABLE).wrapping_add(index.wrapping_mul(4))
                as *const u32)
                .read();
            if bank == 0 {
                (0, 0)
            } else {
                (
                    (bank.wrapping_add(LO) as *const u32).read_unaligned(),
                    (bank.wrapping_add(HI) as *const u32).read_unaligned(),
                )
            }
        };
        (dst as *mut u32).write_unaligned(lo);
        ((dst + 4) as *mut u32).write_unaligned(hi);
        0
    }
});
