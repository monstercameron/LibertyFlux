// original: 0x009AAA80 audio_hash_in_table_a (proposed)

/// Audio hash-membership test a: reports whether the hash of `key`
/// appears in a fixed global table of 52 dwords.
///
/// Calls the shared 32-bit hasher callee with (`key`, 0), then linearly
/// scans the dword table [`TABLE_START`, `TABLE_END`) for the result.
/// Returns 1 on the first match, else 0. The table contents are game data,
/// read but never written here. Returns `al` (the upper bytes of `eax`
/// keep the hasher's leftover bits and are not compared).
/// Original: 0x009AAA80 (stdcall, one stack word).
lf_checker_rt::export!(stdcall, rw_009AAA80(key: u32) -> u8 {
    unsafe {
        const TABLE_START: u32 = 0x01288618;
        const TABLE_END: u32 = 0x012886e8;
        const HASHER: u32 = 1;
        const STEP: u32 = 4;
        let h: u32 = lf_checker_rt::callee_cdecl!(HASHER, u32, key, 0);
        let mut p = lf_checker_rt::relocated(TABLE_START);
        let end = lf_checker_rt::relocated(TABLE_END);
        while p < end {
            if (p as *const u32).read_unaligned() == h {
                return 1;
            }
            p = p.wrapping_add(STEP);
        }
        0
    }
});
