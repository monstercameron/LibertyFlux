// original: 0x008CB3A0 stream_slot_free (proposed)

/// Tests whether streaming slot `index` is free: 1 when the index is
/// non-negative, below the limit (15 when the mode byte `flag` is non-zero,
/// 75 when it is zero) and the slot's in-use byte is 0; otherwise 0.
///
/// Slot `i` owns the byte at `TABLE + i * ROW_STRIDE`. Two stack arguments
/// (cdecl); the byte result is returned in AL.
lf_checker_rt::export!(cdecl, rw_008CB3A0(index: u32, flag: u32) -> u32 {
    unsafe {
        /// Base of the slot in-use bytes.
        const TABLE: u32 = 0x116D398;
        /// Stride between slot bytes.
        const ROW_STRIDE: u32 = 0x134;
        /// Index limit when the mode byte is non-zero.
        const SMALL_LIMIT: u32 = 15;
        /// Index limit when the mode byte is zero.
        const BIG_LIMIT: u32 = 75;
        let limit = if (flag & 0xFF) != 0 { SMALL_LIMIT } else { BIG_LIMIT };
        if (index as i32) < 0 || index >= limit {
            return 0;
        }
        let used = (lf_checker_rt::relocated(TABLE).wrapping_add(index.wrapping_mul(ROW_STRIDE))
            as *const u8)
            .read();
        u32::from(used == 0)
    }
});
