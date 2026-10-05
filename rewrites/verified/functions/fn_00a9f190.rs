// original: 0x00a9f190 stream_find_live_entry (proposed)

/// Find the first of 256 entries whose key matches and whose link is live.
///
/// `this` points to an object holding 256 entries of 0x1c bytes starting at
/// `+0x0`. Entry `i` matches when the word at entry `+0x08` equals `key`,
/// the link word at entry `+0x0c` is non-null, and the word at link
/// `+0x70` is non-zero. The base address of the first matching entry is
/// returned, else null.
///
/// Original: 0x00a9f190 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a9f190(this: u32, key: u32) -> u32 {
    unsafe {
        const KEY_DELTA: u32 = 0x08;
        const LINK_DELTA: u32 = 0x0c;
        const ENTRY_STRIDE: u32 = 0x1c;
        const ENTRY_COUNT: u32 = 0x100;
        const TARGET_STATE_OFF: u32 = 0x70;
        let mut i = 0u32;
        while i < ENTRY_COUNT {
            let e = this.wrapping_add(i.wrapping_mul(ENTRY_STRIDE));
            if ((e + KEY_DELTA) as *const u32).read_unaligned() == key {
                let link = ((e + LINK_DELTA) as *const u32).read_unaligned();
                if link != 0
                    && ((link + TARGET_STATE_OFF) as *const u32).read_unaligned() != 0
                {
                    return e;
                }
            }
            i += 1;
        }
        0
    }
});
