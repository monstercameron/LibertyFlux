// original: 0x00565b20 race87_vf13_key_value

/// Column value for a key in a leaderboard's key/value lists.
///
/// `key` is looked up in the leaderboard identified by `LEADERBOARD_ID`.
/// The loader callee fills `info` (entry count at `+0x0C`, key-list pointer
/// at `+0x10`, value-list pointer at `+0x14`). When the loader reports
/// failure, or the signed count is not positive, or the key is absent, the
/// result is `NOT_FOUND`. Otherwise the value parked next to the matching
/// key is returned.
///
/// Edge cases: a zero or negative count returns `NOT_FOUND` without reading
/// either list; a key that only occurs at or past the bound is not found.
/// The original compares the found position against -1 before indexing the
/// values; the position is never negative there, so the check cannot fire.
///
/// Original: thiscall, one stack word; incoming ECX is ignored.
lf_checker_rt::export!(thiscall, rw_00565b20(_this: u32, key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x122;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        const INFO_COUNT: usize = 3; // +0x0C: number of entries (signed)
        const INFO_KEYS: usize = 4; // +0x10: key list
        const INFO_VALS: usize = 5; // +0x14: value list
        const CALLEE_FILL: u32 = 1;

        let mut info = [0u32; 6];
        let ok: u32 = lf_checker_rt::callee_fastcall!(
            CALLEE_FILL, u32, LEADERBOARD_ID, info.as_mut_ptr() as u32);
        if (ok & 0xFF) == 0 {
            return NOT_FOUND;
        }
        let count = info[INFO_COUNT] as i32;
        if count <= 0 {
            return NOT_FOUND;
        }
        let keys = info[INFO_KEYS];
        let mut i = 0i32;
        while i < count {
            let k = (keys.wrapping_add((i as u32).wrapping_mul(4)) as *const u32).read_unaligned();
            if k == key {
                if (i as u32) == NOT_FOUND {
                    return NOT_FOUND;
                }
                let vals = info[INFO_VALS];
                return (vals.wrapping_add((i as u32).wrapping_mul(4)) as *const u32)
                    .read_unaligned();
            }
            i += 1;
        }
        NOT_FOUND
    }
});
