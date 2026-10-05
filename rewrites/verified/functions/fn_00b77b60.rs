// original: 0x00b77b60 linear_search_9 (proposed)

/// Linear-search nine dwords for `key`, starting at `this+4`.
///
/// Compares `this+4`, `this+8`, ... one at a time and returns the zero-based
/// index of the first match. Returns -1 when none of the nine words equals
/// `key`. Reads exactly nine dwords; writes nothing.
///
/// Original: 0x00b77b60 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00b77b60(this: u32, key: u32) -> u32 {
    unsafe {
        const TABLE_OFF: u32 = 4;
        const COUNT: u32 = 9;
        const MISS: u32 = 0xffff_ffff;
        let mut i = 0u32;
        while i < COUNT {
            let v = ((this + TABLE_OFF + i * 4) as *const u32).read_unaligned();
            if v == key {
                return i;
            }
            i += 1;
        }
        MISS
    }
});
