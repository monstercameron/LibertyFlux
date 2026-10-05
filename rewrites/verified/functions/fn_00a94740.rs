// original: 0x00a94740 stream_entry_slot_index (proposed)

/// Slot number of a stream entry relative to its kind's base slot.
///
/// The entry's index is `(entry - table_base) / 24` (signed, via a magic
/// multiply by `0x2aaaaaab`), truncated to 16 bits, minus the kind's base
/// slot from a global table indexed by the kind byte at `+0x17` times 100.
/// The table base is the global at `0x12fb3a8`. Pure leaf.
///
/// Original: thiscall, no stack arguments.
lf_checker_rt::export!(thiscall, rw_00a94740(this: u32) -> u32 {
    unsafe {
        const TABLE_BASE_G: u32 = 0x012fb3a8;
        const KIND_TABLE: u32 = 0x013053a8;
        const ENT_KIND: u32 = 0x17;
        const KIND_STRIDE: u32 = 100;
        unsafe fn rd32(a: u32) -> u32 {
            unsafe { (a as *const u32).read_unaligned() }
        }
        unsafe fn rd8(a: u32) -> u8 {
            unsafe { (a as *const u8).read() }
        }
        /// Signed divide by 24 as the original's magic sequence does it.
        fn div24(d: i32) -> i32 {
            let hi = ((d as i64 * 0x2aaaaaabi64) >> 32) as i32;
            let s = hi >> 2;
            s.wrapping_add(((s as u32) >> 31) as i32)
        }
        let base = rd32(lf_checker_rt::relocated(TABLE_BASE_G));
        let d = (this as i32).wrapping_sub(base as i32);
        let idx = div24(d) as u16 as u32;
        let kind = rd8(this.wrapping_add(ENT_KIND)) as u32;
        let slot = rd32(lf_checker_rt::relocated(KIND_TABLE)
            .wrapping_add(kind.wrapping_mul(KIND_STRIDE)));
        idx.wrapping_sub(slot)
    }
});
