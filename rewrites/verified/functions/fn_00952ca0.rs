// original: 0x00952CA0 claim_first_free_slot (proposed)

/// Claim the first free 8-byte slot of the 0x818-entry global table.
///
/// Scans `TABLE` upward for the first entry whose first dword is zero. When
/// found, tags it through callee 1 (no arguments; only the low 16 bits of
/// the answer are used), stores `val` into the first dword and the tag into
/// the low word of the second, and returns (`tag` << 16) | index. When every
/// entry is busy the return is -1. All bounds are signed but stay positive.
///
/// Original: 0x00952CA0 (cdecl, one stack word).
lf_checker_rt::export!(cdecl, rw_00952CA0(val: u32) -> u32 {
    unsafe {
        const TABLE: u32 = 0x12142D0;
        const SLOTS: u32 = 0x818;
        const TAG: u32 = 1;
        const FULL: u32 = 0xFFFF_FFFF;
        let mut i = 0u32;
        loop {
            if i >= SLOTS {
                return FULL;
            }
            let p = lf_checker_rt::relocated(TABLE).wrapping_add(i.wrapping_mul(8));
            if (p as *const u32).read() == 0 {
                let t = lf_checker_rt::callee_cdecl!(TAG, u32,) & 0xFFFF;
                (p as *mut u32).write(val);
                (p.wrapping_add(4) as *mut u16).write_unaligned(t as u16);
                return (t << 16) | i;
            }
            i += 1;
        }
    }
});
