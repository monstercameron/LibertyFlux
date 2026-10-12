// original: 0x0088A560 stream_resource_find_largest_pending_group (proposed)

/// Index of the group with the most pending chunks (first one on ties).
///
/// `this` points to the resource manager: group count range from `+0xcc`
/// minus `+0xe0`, table base at `+0xe8`. Each group's pending count is the
/// byte at `base + 0x6f0d + i * 0x6f40`. Returns the index holding the
/// largest byte (strictly greater wins, so the first maximum is kept), or 0
/// when the range is empty. The loop bound is compared unsigned.
///
/// Original: 0x0088A560 (thiscall, `this` in ecx, no stack words).
lf_checker_rt::export!(thiscall, rw_0088a560(this: u32) -> u32 {
    unsafe {
        const RANGE_HI: u32 = 0xcc;
        const RANGE_LO: u32 = 0xe0;
        const TABLE_BASE: u32 = 0xe8;
        const COUNT_OFF: u32 = 0x6f0d;
        const GROUP_STRIDE: u32 = 0x6f40;

        let n = ((this + RANGE_HI) as *const u32)
            .read_unaligned()
            .wrapping_sub(((this + RANGE_LO) as *const u32).read_unaligned());
        if n == 0 {
            return 0;
        }
        let base = ((this + TABLE_BASE) as *const u32).read_unaligned();
        let mut best_idx = 0u32;
        let mut best_val = 0u32;
        let mut i = 0u32;
        while i < n {
            let v = ((base
                .wrapping_add(COUNT_OFF)
                .wrapping_add(i.wrapping_mul(GROUP_STRIDE))) as *const u8)
                .read() as u32;
            if v > best_val {
                best_val = v;
                best_idx = i;
            }
            i = i.wrapping_add(1);
        }
        best_idx
    }
});
