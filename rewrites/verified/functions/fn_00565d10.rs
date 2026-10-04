// original: 0x00565d10 race87_vf6_key_position

/// Position of a key inside a leaderboard column's key list.
///
/// `key` is looked up in the leaderboard identified by `LEADERBOARD_ID`.
/// The loader callee fills `info` (entry count at `+0x0C`, key-list pointer
/// at `+0x10`). When the loader reports failure, or the signed count is not
/// positive, the result is `NOT_FOUND`. Otherwise the list is scanned with
/// a signed bound and the first matching position is returned.
///
/// Edge cases: a zero or negative count returns `NOT_FOUND` without reading
/// the list; a key that only occurs at or past the bound is not found.
///
/// Original: thiscall, one stack word; incoming ECX is ignored.
lf_checker_rt::export!(thiscall, rw_00565d10(_this: u32, key: u32) -> u32 {
    unsafe {
        const LEADERBOARD_ID: u32 = 0x122;
        const NOT_FOUND: u32 = 0xFFFF_FFFF;
        const INFO_COUNT: usize = 3; // +0x0C: number of keys (signed)
        const INFO_KEYS: usize = 4; // +0x10: key list
        const CALLEE_FILL: u32 = 1;

        let mut info = [0u32; 5];
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
                return i as u32;
            }
            i += 1;
        }
        NOT_FOUND
    }
});
