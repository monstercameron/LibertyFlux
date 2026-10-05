// original: 0x00a9f1e0 stream_find_entry_by_key (proposed)

/// Find the first of 256 entries whose key word equals `key`.
///
/// `this` points to an object holding 256 entries of 0x1c bytes starting at
/// `+0x0`; the compared key is the word at entry `+0x0c`. Entries are
/// scanned in order and the base address of the first match is returned
/// (note: the entry base, not the key address). No match gives null.
///
/// Original: 0x00a9f1e0 (thiscall, one stack word).
lf_checker_rt::export!(thiscall, rw_00a9f1e0(this: u32, key: u32) -> u32 {
    unsafe {
        const KEY_DELTA: u32 = 0x0c;
        const ENTRY_STRIDE: u32 = 0x1c;
        const ENTRY_COUNT: u32 = 0x100;
        let mut i = 0u32;
        while i < ENTRY_COUNT {
            let e = this.wrapping_add(i.wrapping_mul(ENTRY_STRIDE));
            if ((e + KEY_DELTA) as *const u32).read_unaligned() == key {
                return e;
            }
            i += 1;
        }
        0
    }
});
