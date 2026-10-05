// original: 0x00a45490 vehicle_find_subpart_index
/// Index of the first 0x14-byte record whose head word equals `want`.
///
/// The record count is at `this+0xFA8`, the records start at `this+0xFB8`
/// (thiscall, one stack word). Returns the zero-based index, or -1 when no
/// record matches or the count is zero. The unsigned loop bound means a
/// huge count reads out of bounds and faults, on both sides alike.
export!(thiscall, rw_00a45490(this: u32, want: u32) -> u32 {
    unsafe {
        const COUNT_OFF: u32 = 0xfa8;
        const RECS_OFF: u32 = 0xfb8;
        const STRIDE: u32 = 0x14;
        let count = (this.wrapping_add(COUNT_OFF) as *const u32).read_unaligned();
        let mut p = this.wrapping_add(RECS_OFF);
        let mut i = 0u32;
        while i < count {
            if (p as *const u32).read_unaligned() == want {
                return i;
            }
            p = p.wrapping_add(STRIDE);
            i += 1;
        }
        0xFFFFFFFF
    }
});
