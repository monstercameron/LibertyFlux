// original: 0x0052db40 leaderboard_map_key_to_value

/// Map a leaderboard key to its value through the schema arrays.
///
/// `rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race25Standard, player_schema::LeaderboardInfo, 10>::vf13`.
///
/// `key` is compared against the key array the fetch callee reports (schema
/// kind `KIND`): the six-word out-struct carries the signed match count at
/// word `OUT_COUNT` (+0xc), the key array at `OUT_KEYS` (+0x10) and the value
/// array at `OUT_VALS` (+0x14). A failed fetch, a non-positive count, or no
/// match yields the `MISSING` sentinel; otherwise the result is the value at
/// the matched position.
///
/// Calling convention: thiscall slot, `this` ignored, one stack argument,
/// callee pops 4. The fetch callee pops nothing and answers in AL.
lf_checker_rt::export!(thiscall, rw_0052db40(_this: u32, key: u32) -> u32 {
    /// Schema-table selector passed to the fetch callee in ECX.
    const KIND: u32 = 0x2a;
    /// Out-struct words: signed match count (+0xc), key array (+0x10), value array (+0x14).
    const OUT_COUNT: usize = 3;
    const OUT_KEYS: usize = 4;
    const OUT_VALS: usize = 5;
    /// Failure sentinel.
    const MISSING: u32 = 0xffff_ffff;
    let mut out = [0u32; 6];
    out[OUT_COUNT] = 0;
    out[OUT_KEYS] = 0;
    out[OUT_VALS] = 0;
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
            let vals = out[OUT_VALS];
            return unsafe { (vals.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned() };
        }
        i = i.wrapping_add(1);
        if (i as i32) >= count {
            return MISSING;
        }
    }
});
