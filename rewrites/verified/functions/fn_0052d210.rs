// original: 0x0052d210 leaderboard_find_value_index

/// Find a schema value inside a second array, by index.
///
/// `rage::rlConcreteLeaderboardInfo<player_schema::Leaderboard_Ranked_Race23Standard, player_schema::LeaderboardInfo, 10>::vf12`.
///
/// `index` selects a dword from the value array the fetch callee reports
/// (schema kind `KIND`): the six-word out-struct carries the match count at
/// word `OUT_COUNT` (+0x4), the searched array at `OUT_KEYS` (+0x8) and the
/// value array at `OUT_VALS` (+0x14). A failed fetch, a -1 value, a zero
/// count, or no match yields the `MISSING` sentinel; otherwise the result is
/// the unsigned position of the first equal dword.
///
/// Calling convention: thiscall slot, `this` ignored, one stack argument,
/// callee pops 4. The fetch callee pops nothing and answers in AL.
lf_checker_rt::export!(thiscall, rw_0052d210(_this: u32, index: u32) -> u32 {
    /// Schema-table selector passed to the fetch callee in ECX.
    const KIND: u32 = 0x1c;
    /// Out-struct words: match count (+0x4), searched array (+0x8), value array (+0x14).
    const OUT_COUNT: usize = 1;
    const OUT_KEYS: usize = 2;
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
    let vals = out[OUT_VALS];
    let want = unsafe { (vals.wrapping_add(index.wrapping_mul(4)) as *const u32).read_unaligned() };
    if want == MISSING {
        return MISSING;
    }
    let count = out[OUT_COUNT];
    if count == 0 {
        return MISSING;
    }
    let keys = out[OUT_KEYS];
    let mut i = 0u32;
    loop {
        let cur = unsafe { (keys.wrapping_add(i.wrapping_mul(4)) as *const u32).read_unaligned() };
        if cur == want {
            return i;
        }
        i = i.wrapping_add(1);
        if i >= count {
            return MISSING;
        }
    }
});
