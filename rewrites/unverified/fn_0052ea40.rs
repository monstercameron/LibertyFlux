// original: 0x0052ea40 leaderboard_find_key_index

/// Find a leaderboard key inside the schema key array.
///
/// `rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race28Standard, player_schema::LeaderboardInfo, 10>::vf6`.
///
/// `key` is compared against the key array the fetch callee reports (schema
/// kind `KIND`): the six-word out-struct carries the signed match count at
/// word `OUT_COUNT` (+0xc) and the key array at `OUT_KEYS` (+0x10). A failed
/// fetch, a non-positive count, or no match yields the `MISSING` sentinel;
/// otherwise the result is the signed position of the first equal dword.
///
/// Calling convention: thiscall slot, `this` ignored, one stack argument,
/// callee pops 4. The fetch callee pops nothing and answers in AL.
lf_checker_rt::export!(thiscall, rw_0052ea40(_this: u32, key: u32) -> u32 {
    /// Schema-table selector passed to the fetch callee in ECX.
    const KIND: u32 = 0x5;
    /// Out-struct words: signed match count (+0xc), key array (+0x10).
    const OUT_COUNT: usize = 3;
    const OUT_KEYS: usize = 4;
    /// Failure sentinel.
    const MISSING: u32 = 0xffff_ffff;
    let mut out = [0u32; 6];
    out[OUT_COUNT] = 0;
    out[OUT_KEYS] = 0;
    let ok = lf_checker_rt::callee_fastcall!(1, u32, KIND, out.as_mut_ptr() as u32);
    if ok & 0xFF == 0 {
        return MISSING;
    }
    let count = out[OUT_COUNT] as i32;
    if count <= 0 {
        return MISSING;
    }
    let keys = out[OUT_KEYS];
    let mut i = 0u32;
    loop {
        let cur = unsafe { (keys.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned() };
        if cur == key {
            return i;
        }
        i = i.wrapping_add(1);
        if (i as i32) >= count {
            return MISSING;
        }
    }
});
